#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod smh_registry {
    use ink::storage::Mapping;
    use ink::prelude::string::String;
    use ink::H160;

    #[ink(event)]
    pub struct DomainRegistered {
        pub domain: String,
    }

    #[ink(event)]
    pub struct DomainExtentionCreated {
        pub extention: String,
    }

    #[ink(event)]
    pub struct ZoneUpdated {
        #[ink(topic)]
        domain: String,
        #[ink(topic)]
        name: String,
        #[ink(topic)]
        value: String,
    }

    #[ink(event)]
    pub struct OwnershipTransferred {
        #[ink(topic)]
        domain: String,
        #[ink(topic)]
        new_owner: H160,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct SmhDomains {
        owners: Mapping<String, H160>,
        zones: Mapping<(String, String), String>,
        extention: String
    }

    impl SmhDomains {
        #[ink(constructor)]
        pub fn extention_init(extention: String) -> Self {
            let instance = Self {
                owners: Mapping::default(),
                zones: Mapping::default(),
                extention: extention.clone(),
            };
            instance.env().emit_event(DomainExtentionCreated {
                extention
            });
            instance
        }

        #[ink(message)]
        pub fn register_domain(&mut self, domain: String) -> bool {
            let caller = Self::env().caller();
            assert!(self.owners.get(&domain).is_none(), "Domain already exists");
            self.owners.insert(&domain, &caller);
            self.env().emit_event(DomainRegistered {
                domain
            });
            true
        }

        #[ink(message)]
        pub fn add_zone(&mut self, domain: String, name: String, value: String) {
            let caller = Self::env().caller();
            let owner = self.owners.get(&domain).expect("Domain not registered");
            assert!(caller == owner, "You are not the owner");

            self.zones.insert(&(domain.clone(), name.clone()), &value);

            Self::env().emit_event(ZoneUpdated {
                domain: domain.clone(),
                name: name.clone(),
                value: value.clone(),
            });
        }

        #[ink(message)]
        pub fn update_zone(&mut self, domain: String, name: String, value: String) -> bool {
            let caller = Self::env().caller();
            let owner = self.owners.get(&domain).expect("Domain not registered");
            assert!(caller == owner, "You are not the owner");

            self.zones.insert(&(domain.clone(), name.clone()), &value);

            Self::env().emit_event(ZoneUpdated {
                domain,
                name,
                value,
            });

            true
        }

        #[ink(message)]
        pub fn transfer_domain(&mut self, domain: String, new_owner: H160) -> bool {
            let caller = Self::env().caller();
            let owner = self.owners.get(&domain).expect("Domain not registered");
            assert!(caller == owner, "You are not the owner");
            assert!(new_owner != owner, "You cannot transfer to yourself");

            self.owners.insert(&domain, &new_owner);

            Self::env().emit_event(OwnershipTransferred {
                domain,
                new_owner,
            });
            true
        }

        #[ink(message)]
        pub fn owner_of(&self, domain: String) -> Option<H160> {
            self.owners.get(&domain)
        }

        #[ink(message)]
        pub fn extention(&self) -> String {
            self.extention.clone()
        }

        #[ink(message)]
        pub fn get_zone(&self, domain: String, name: String) -> String {
            self.zones
                .get(&(domain, name))
                .unwrap_or(String::from("Zone not found"))
        }
    }
}