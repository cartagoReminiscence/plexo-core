use plexo_sdk::labels::label::Label;
use plexo_sdk::labels::operations::{
    CreateLabelInput, GetLabelsInput, LabelCrudOperations, UpdateLabelInput,
};
use plexo_sdk::members::member::Member;
use plexo_sdk::members::operations::{
    CreateMemberInput, GetMembersInput, MemberCrudOperations, UpdateMemberInput,
};
use plexo_sdk::projects::operations::{
    CreateProjectInput, GetProjectsInput, ProjectCrudOperations, UpdateProjectInput,
};
use plexo_sdk::tasks::operations::{
    CreateTaskInput, GetTasksInput, TaskCrudOperations, UpdateTaskInput,
};
use plexo_sdk::teams::operations::{
    CreateTeamInput, GetTeamsInput, TeamCrudOperations, UpdateTeamInput,
};
use plexo_sdk::teams::team::Team;
use plexo_sdk::{projects::project::Project, tasks::task::Task};
use poem::Result;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi::{ApiResponse, OpenApi, Tags};
use uuid::Uuid;

use crate::core::app::Core;
use crate::errors::app::PlexoAppError;

use super::auth::PlexoAPIKeyAuthorization;

#[derive(Tags)]
enum PlexoAPITags {
    /// Operations about tasks
    Task,
    /// Operations about projects
    Project,
    /// Operations about members
    Member,
    /// Operations about teams
    Team,
    /// Operations about labels
    Label,
}

pub struct PlexoOpenAPI {
    pub core: Core,
}

impl PlexoOpenAPI {
    pub fn new(core: Core) -> Self {
        Self { core }
    }
}

#[OpenApi]
impl PlexoOpenAPI {
    #[oai(
        path = "/tasks",
        method = "post",
        tag = "PlexoAPITags::Task",
        operation_id = "create_task"
    )]
    async fn create_task(
        &self,
        mut input: Json<CreateTaskInput>,
        auth: PlexoAPIKeyAuthorization,
    ) -> Result<CreateTaskResponse> {
        input.owner_id = auth.0.member_id();

        let task = self
            .core
            .engine
            .create_task(input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(CreateTaskResponse::Ok(Json(task)))
    }

    #[oai(
        path = "/tasks/:id",
        method = "get",
        tag = "PlexoAPITags::Task",
        operation_id = "get_task"
    )]
    async fn get_task(
        &self,
        id: Path<Uuid>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<GetTaskResponse> {
        let task = self
            .core
            .engine
            .get_task(id.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(GetTaskResponse::Ok(Json(task)))
    }

    #[oai(
        path = "/tasks",
        method = "get",
        tag = "PlexoAPITags::Task",
        operation_id = "get_tasks"
    )]
    async fn get_tasks(
        &self,
        input: Json<GetTasksInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<GetTasksResponse> {
        let tasks = self
            .core
            .engine
            .get_tasks(Some(input.0))
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(GetTasksResponse::Ok(Json(tasks)))
    }

    #[oai(
        path = "/tasks/:id",
        method = "put",
        tag = "PlexoAPITags::Task",
        operation_id = "update_task"
    )]
    async fn update_task(
        &self,
        id: Path<Uuid>,
        input: Json<UpdateTaskInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<UpdateTaskResponse> {
        let task = self
            .core
            .engine
            .update_task(id.0, input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(UpdateTaskResponse::Ok(Json(task)))
    }

    #[oai(
        path = "/tasks/:id",
        method = "delete",
        tag = "PlexoAPITags::Task",
        operation_id = "delete_task"
    )]
    async fn delete_task(
        &self,
        id: Path<Uuid>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<DeleteTaskResponse> {
        let task = self
            .core
            .engine
            .delete_task(id.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(DeleteTaskResponse::Ok(Json(task)))
    }

    #[oai(
        path = "/projects",
        method = "post",
        tag = "PlexoAPITags::Project",
        operation_id = "create_project"
    )]
    async fn create_project(
        &self,
        mut input: Json<CreateProjectInput>,
        auth: PlexoAPIKeyAuthorization,
    ) -> Result<CreateProjectResponse> {
        input.owner_id = auth.0.member_id();

        let project = self
            .core
            .engine
            .create_project(input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(CreateProjectResponse::Ok(Json(project)))
    }

    #[oai(
        path = "/projects/:id",
        method = "get",
        tag = "PlexoAPITags::Project",
        operation_id = "get_project"
    )]
    async fn get_project(
        &self,
        id: Path<Uuid>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<GetProjectResponse> {
        let project = self
            .core
            .engine
            .get_project(id.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(GetProjectResponse::Ok(Json(project)))
    }

    #[oai(
        path = "/projects",
        method = "get",
        tag = "PlexoAPITags::Project",
        operation_id = "get_projects"
    )]
    async fn get_projects(
        &self,
        input: Json<GetProjectsInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> GetProjectsResponse {
        let projects = self
            .core
            .engine
            .get_projects(input.0)
            .await
            .map_err(PlexoAppError::SDKError)
            .unwrap();

        GetProjectsResponse::Ok(Json(projects))
    }

    #[oai(
        path = "/projects/:id",
        method = "put",
        tag = "PlexoAPITags::Project",
        operation_id = "update_project"
    )]
    async fn update_project(
        &self,
        id: Path<Uuid>,
        input: Json<UpdateProjectInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<UpdateProjectResponse> {
        let project = self
            .core
            .engine
            .update_project(id.0, input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(UpdateProjectResponse::Ok(Json(project)))
    }

    #[oai(
        path = "/projects/:id",
        method = "delete",
        tag = "PlexoAPITags::Project",
        operation_id = "delete_project"
    )]
    async fn delete_project(
        &self,
        id: Path<Uuid>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<DeleteProjectResponse> {
        let project = self
            .core
            .engine
            .delete_project(id.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(DeleteProjectResponse::Ok(Json(project)))
    }

    #[oai(
        path = "/members",
        method = "post",
        tag = "PlexoAPITags::Member",
        operation_id = "create_member"
    )]
    async fn create_member(
        &self,
        input: Json<CreateMemberInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<CreateMemberResponse> {
        let member = self
            .core
            .engine
            .create_member(input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(CreateMemberResponse::Ok(Json(member)))
    }

    #[oai(
        path = "/members/:id",
        method = "get",
        tag = "PlexoAPITags::Member",
        operation_id = "get_member"
    )]
    async fn get_member(
        &self,
        id: Path<Uuid>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<GetMemberResponse> {
        let member = self
            .core
            .engine
            .get_member(id.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(GetMemberResponse::Ok(Json(member)))
    }

    #[oai(
        path = "/members",
        method = "get",
        tag = "PlexoAPITags::Member",
        operation_id = "get_members"
    )]
    async fn get_members(
        &self,
        input: Json<GetMembersInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<GetMembersResponse> {
        let members = self
            .core
            .engine
            .get_members(input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(GetMembersResponse::Ok(Json(members)))
    }

    #[oai(
        path = "/members/:id",
        method = "put",
        tag = "PlexoAPITags::Member",
        operation_id = "update_member"
    )]
    async fn update_member(
        &self,
        id: Path<Uuid>,
        input: Json<UpdateMemberInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<UpdateMemberResponse> {
        let member = self
            .core
            .engine
            .update_member(id.0, input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(UpdateMemberResponse::Ok(Json(member)))
    }

    #[oai(
        path = "/members/:id",
        method = "delete",
        tag = "PlexoAPITags::Member",
        operation_id = "delete_member"
    )]
    async fn delete_member(
        &self,
        id: Path<Uuid>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<DeleteMemberResponse> {
        let member = self
            .core
            .engine
            .delete_member(id.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(DeleteMemberResponse::Ok(Json(member)))
    }

    #[oai(
        path = "/teams",
        method = "post",
        tag = "PlexoAPITags::Team",
        operation_id = "create_team"
    )]
    async fn create_team(
        &self,
        input: Json<CreateTeamInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<CreateTeamResponse> {
        let team = self
            .core
            .engine
            .create_team(input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(CreateTeamResponse::Ok(Json(team)))
    }

    #[oai(
        path = "/teams/:id",
        method = "get",
        tag = "PlexoAPITags::Team",
        operation_id = "get_team"
    )]
    async fn get_team(
        &self,
        id: Path<Uuid>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<GetTeamResponse> {
        let team = self
            .core
            .engine
            .get_team(id.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(GetTeamResponse::Ok(Json(team)))
    }

    #[oai(
        path = "/teams",
        method = "get",
        tag = "PlexoAPITags::Team",
        operation_id = "get_teams"
    )]
    async fn get_teams(
        &self,
        input: Json<GetTeamsInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<GetTeamsResponse> {
        let teams = self
            .core
            .engine
            .get_teams(input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(GetTeamsResponse::Ok(Json(teams)))
    }

    #[oai(
        path = "/teams/:id",
        method = "put",
        tag = "PlexoAPITags::Team",
        operation_id = "update_team"
    )]
    async fn update_team(
        &self,
        id: Path<Uuid>,
        input: Json<UpdateTeamInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<UpdateTeamResponse> {
        let team = self
            .core
            .engine
            .update_team(id.0, input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(UpdateTeamResponse::Ok(Json(team)))
    }

    #[oai(
        path = "/teams/:id",
        method = "delete",
        tag = "PlexoAPITags::Team",
        operation_id = "delete_team"
    )]
    async fn delete_team(
        &self,
        id: Path<Uuid>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<DeleteTeamResponse> {
        let team = self
            .core
            .engine
            .delete_team(id.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(DeleteTeamResponse::Ok(Json(team)))
    }

    #[oai(
        path = "/labels",
        method = "post",
        tag = "PlexoAPITags::Label",
        operation_id = "create_label"
    )]
    async fn create_label(
        &self,
        input: Json<CreateLabelInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<CreateLabelResponse> {
        let label = self
            .core
            .engine
            .create_label(input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(CreateLabelResponse::Ok(Json(label)))
    }

    #[oai(
        path = "/labels/:id",
        method = "get",
        tag = "PlexoAPITags::Label",
        operation_id = "get_label"
    )]
    async fn get_label(
        &self,
        id: Path<Uuid>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<GetLabelResponse> {
        let label = self
            .core
            .engine
            .get_label(id.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(GetLabelResponse::Ok(Json(label)))
    }

    #[oai(
        path = "/labels",
        method = "get",
        tag = "PlexoAPITags::Label",
        operation_id = "get_labels"
    )]
    async fn get_labels(
        &self,
        input: Json<GetLabelsInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<GetLabelsResponse> {
        let labels = self
            .core
            .engine
            .get_labels(input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(GetLabelsResponse::Ok(Json(labels)))
    }

    #[oai(
        path = "/labels/:id",
        method = "put",
        tag = "PlexoAPITags::Label",
        operation_id = "update_label"
    )]
    async fn update_label(
        &self,
        id: Path<Uuid>,
        input: Json<UpdateLabelInput>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<UpdateLabelResponse> {
        let label = self
            .core
            .engine
            .update_label(id.0, input.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(UpdateLabelResponse::Ok(Json(label)))
    }

    #[oai(
        path = "/labels/:id",
        method = "delete",
        tag = "PlexoAPITags::Label",
        operation_id = "delete_label"
    )]
    async fn delete_label(
        &self,
        id: Path<Uuid>,
        _auth: PlexoAPIKeyAuthorization,
    ) -> Result<DeleteLabelResponse> {
        let label = self
            .core
            .engine
            .delete_label(id.0)
            .await
            .map_err(PlexoAppError::SDKError)?;

        Ok(DeleteLabelResponse::Ok(Json(label)))
    }
}

//
//
//

//
//
//

#[derive(ApiResponse)]
enum CreateProjectResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Project>),
}

#[derive(ApiResponse)]
enum GetProjectsResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Vec<Project>>),
}

#[derive(ApiResponse)]
enum GetProjectResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Project>),
}

#[derive(ApiResponse)]
enum UpdateProjectResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Project>),
}

#[derive(ApiResponse)]
enum DeleteProjectResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Project>),
}

//
//
//

//
//
//

#[derive(ApiResponse)]
enum CreateTaskResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Task>),
}

#[derive(ApiResponse)]
enum GetTasksResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Vec<Task>>),
}

#[derive(ApiResponse)]
enum GetTaskResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Task>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum UpdateTaskResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Task>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum DeleteTaskResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Task>),
    // #[oai(status = 404)]
    // NotFound,
}

//
//
//

//
//
//

#[derive(ApiResponse)]
enum CreateMemberResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Member>),
}

#[derive(ApiResponse)]
enum GetMembersResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Vec<Member>>),
}

#[derive(ApiResponse)]
enum GetMemberResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Member>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum UpdateMemberResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Member>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum DeleteMemberResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Member>),
    // #[oai(status = 404)]
    // NotFound,
}

//
//
//

//
//
//

#[derive(ApiResponse)]
enum CreateTeamResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Team>),
}

#[derive(ApiResponse)]
enum GetTeamsResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Vec<Team>>),
}

#[derive(ApiResponse)]
enum GetTeamResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Team>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum UpdateTeamResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Team>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum DeleteTeamResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Team>),
    // #[oai(status = 404)]
    // NotFound,
}

//
//
//

//
//
//

#[derive(ApiResponse)]
enum CreateLabelResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Label>),
}

#[derive(ApiResponse)]
enum GetLabelsResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Vec<Label>>),
}

#[derive(ApiResponse)]
enum GetLabelResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Label>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum UpdateLabelResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Label>),
    // #[oai(status = 404)]
    // NotFound,
}

#[derive(ApiResponse)]
enum DeleteLabelResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok(Json<Label>),
    // #[oai(status = 404)]
    // NotFound,
}
