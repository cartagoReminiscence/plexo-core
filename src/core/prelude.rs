use super::{
    app::Core,
    config::{
        ADMIN_EMAIL, ADMIN_NAME, ADMIN_PASSWORD, ADMIN_PHOTO_URL, ORGANIZATION_EMAIL, ORGANIZATION_NAME, ORGANIZATION_PHOTO_URL,
    },
};

use plexo_sdk::{
    common::commons::SortOrder,
    organization::operations::{Organization, OrganizationCrudOperations, OrganizationInitializationInputBuilder},
    resources::members::{
        extensions::{CreateMemberFromEmailInputBuilder, MembersExtensionOperations},
        member::MemberRole,
        operations::{GetMembersInput, GetMembersInputBuilder, MemberCrudOperations},
    },
};

impl Core {
    pub async fn prelude(&self) -> Result<Organization, Box<dyn std::error::Error>> {
        self.normalize_admin_user().await?;

        match self.engine.get_organization().await? {
            Some(organization) => Ok(organization),
            None => self.initialize_organization().await,
        }
    }

    async fn normalize_admin_user(&self) -> Result<(), Box<dyn std::error::Error>> {
        let default_admin_email = (*ADMIN_EMAIL).clone();
        let default_admin_password = (*ADMIN_PASSWORD).clone();
        let default_admin_name = (*ADMIN_NAME).clone();
        let default_admin_photo_url = (*ADMIN_PHOTO_URL).clone();

        let hashed_password = self.auth.hash_password(default_admin_password.as_str());
        let default_admin_role = MemberRole::Admin;

        match self.engine.get_member_by_email(default_admin_email.clone()).await {
            Ok(Some(_admin)) => {
                println!("Default admin user already exists: {}", default_admin_email);
                return Ok(());
            }
            Err(e) => {
                println!("Error checking for default admin user: {}", e);
                return Err(Box::new(e));
            }
            _ => {}
        }

        if !self.engine.get_members(GetMembersInput::default()).await?.is_empty() {
            println!("Members already exist, skipping default admin user creation");
            return Ok(());
        }

        println!("Creating default admin user: {}", default_admin_email);

        self.engine
            .create_member_from_email(
                CreateMemberFromEmailInputBuilder::default()
                    .email(default_admin_email)
                    .name(default_admin_name)
                    .photo_url(default_admin_photo_url)
                    .role(default_admin_role)
                    .password_hash(hashed_password)
                    .build()?,
            )
            .await?;
        Ok(())
    }

    async fn initialize_organization(&self) -> Result<Organization, Box<dyn std::error::Error>> {
        let members = self
            .engine
            .get_members(
                GetMembersInputBuilder::default()
                    .limit(1)
                    .sort_by("created_at".to_string())
                    .sort_order(SortOrder::Asc)
                    .build()?,
            )
            .await?;

        let first_member = members.first().unwrap();

        let org = self
            .engine
            .initialize_organization(
                first_member.id,
                OrganizationInitializationInputBuilder::default()
                    .name((*ORGANIZATION_NAME).to_owned())
                    .email((*ORGANIZATION_EMAIL).to_owned())
                    .photo_url((*ORGANIZATION_PHOTO_URL).to_owned())
                    .owner_id(first_member.id)
                    .build()?,
            )
            .await
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

        let org_email = org.email.clone();

        self.first_time_welcome_email(org_email)?;

        Ok(org)
    }

    fn first_time_welcome_email(&self, organization_owner_email: String) -> Result<(), Box<dyn std::error::Error>> {
        let from = "onboarding@plexo.app";
        let to = &[organization_owner_email.as_str()];
        let subject = "Welcome to Plexo!";
        let html = "<h1>Welcome to Plexo!</h1>";

        self.send_email(from, to, subject, html).map_err(|err| err.into())
    }
}
