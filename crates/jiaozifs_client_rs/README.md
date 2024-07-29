# Rust API client for openapi

jiaozifs HTTP API


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Generator version: 7.8.0-SNAPSHOT
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost:34913/api/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AksksApi* | [**create_aksk**](docs/AksksApi.md#create_aksk) | **POST** /users/aksk | create aksk
*AksksApi* | [**delete_aksk**](docs/AksksApi.md#delete_aksk) | **DELETE** /users/aksk | delete aksk
*AksksApi* | [**get_aksk**](docs/AksksApi.md#get_aksk) | **GET** /users/aksk | get aksk
*AksksApi* | [**list_aksks**](docs/AksksApi.md#list_aksks) | **GET** /users/aksks | list aksks
*AuthApi* | [**get_user_info**](docs/AuthApi.md#get_user_info) | **GET** /users/user | get information of the currently logged-in user
*AuthApi* | [**login**](docs/AuthApi.md#login) | **POST** /auth/login | perform a login
*AuthApi* | [**logout**](docs/AuthApi.md#logout) | **POST** /auth/logout | perform a logout
*AuthApi* | [**refresh_token**](docs/AuthApi.md#refresh_token) | **GET** /users/refreshtoken | refresh token for more time
*AuthApi* | [**register**](docs/AuthApi.md#register) | **POST** /users/register | perform user registration
*BranchesApi* | [**create_branch**](docs/BranchesApi.md#create_branch) | **POST** /repos/{owner}/{repository}/branch | create branch
*BranchesApi* | [**delete_branch**](docs/BranchesApi.md#delete_branch) | **DELETE** /repos/{owner}/{repository}/branch | delete branch
*BranchesApi* | [**get_branch**](docs/BranchesApi.md#get_branch) | **GET** /repos/{owner}/{repository}/branch | get branch
*BranchesApi* | [**list_branches**](docs/BranchesApi.md#list_branches) | **GET** /repos/{owner}/{repository}/branches | list branches
*CommitApi* | [**compare_commit**](docs/CommitApi.md#compare_commit) | **GET** /repos/{owner}/{repository}/compare/{basehead} | compare two commit
*CommitApi* | [**get_commit_changes**](docs/CommitApi.md#get_commit_changes) | **GET** /repos/{owner}/{repository}/changes/{commit_id} | get changes in commit
*CommitApi* | [**get_entries_in_ref**](docs/CommitApi.md#get_entries_in_ref) | **GET** /repos/{owner}/{repository}/contents | list entries in ref
*CommonApi* | [**get_setup_state**](docs/CommonApi.md#get_setup_state) | **GET** /setup | check if jiaozifs setup
*CommonApi* | [**get_version**](docs/CommonApi.md#get_version) | **GET** /version | return program and runtime version
*GroupApi* | [**list_repo_group**](docs/GroupApi.md#list_repo_group) | **GET** /groups/repo | list groups for repo
*ListMembersApi* | [**list_members**](docs/ListMembersApi.md#list_members) | **GET** /repos/{owner}/{repository}/members | get list of members in repository
*MemberApi* | [**invite_member**](docs/MemberApi.md#invite_member) | **POST** /repos/{owner}/{repository}/member/invite | invite member
*MemberApi* | [**revoke_member**](docs/MemberApi.md#revoke_member) | **DELETE** /repos/{owner}/{repository}/member | Revoke member in repository
*MemberApi* | [**update_member_group**](docs/MemberApi.md#update_member_group) | **POST** /repos/{owner}/{repository}/member | update member by user id and change group role
*MergerequestApi* | [**create_merge_request**](docs/MergerequestApi.md#create_merge_request) | **POST** /repos/{owner}/{repository}/mergerequest | create merge request
*MergerequestApi* | [**get_merge_request**](docs/MergerequestApi.md#get_merge_request) | **GET** /repos/{owner}/{repository}/mergerequest/{mrSeq} | get merge request
*MergerequestApi* | [**list_merge_requests**](docs/MergerequestApi.md#list_merge_requests) | **GET** /repos/{owner}/{repository}/mergerequest | get list of merge request in repository
*MergerequestApi* | [**merge**](docs/MergerequestApi.md#merge) | **POST** /repos/{owner}/{repository}/mergerequest/{mrSeq}/merge | merge a mergerequest
*MergerequestApi* | [**update_merge_request**](docs/MergerequestApi.md#update_merge_request) | **POST** /repos/{owner}/{repository}/mergerequest/{mrSeq} | update merge request
*ObjectsApi* | [**delete_object**](docs/ObjectsApi.md#delete_object) | **DELETE** /object/{owner}/{repository} | delete object. Missing objects will not return a NotFound error.
*ObjectsApi* | [**get_files**](docs/ObjectsApi.md#get_files) | **GET** /object/{owner}/{repository}/files | get files by pattern
*ObjectsApi* | [**get_object**](docs/ObjectsApi.md#get_object) | **GET** /object/{owner}/{repository} | get object content
*ObjectsApi* | [**head_object**](docs/ObjectsApi.md#head_object) | **HEAD** /object/{owner}/{repository} | check if object exists
*ObjectsApi* | [**upload_object**](docs/ObjectsApi.md#upload_object) | **POST** /object/{owner}/{repository} | 
*RepoApi* | [**change_visible**](docs/RepoApi.md#change_visible) | **POST** /repos/{owner}/{repository}/visible | change repository visible(true for public, false for private)
*RepoApi* | [**create_repository**](docs/RepoApi.md#create_repository) | **POST** /users/repos | create repository
*RepoApi* | [**delete_repository**](docs/RepoApi.md#delete_repository) | **DELETE** /repos/{owner}/{repository} | delete repository
*RepoApi* | [**get_commits_in_ref**](docs/RepoApi.md#get_commits_in_ref) | **GET** /repos/{owner}/{repository}/commits | get commits in ref
*RepoApi* | [**get_repository**](docs/RepoApi.md#get_repository) | **GET** /repos/{owner}/{repository} | get repository
*RepoApi* | [**list_public_repository**](docs/RepoApi.md#list_public_repository) | **GET** /repos/public | list public repository in all system
*RepoApi* | [**list_repository**](docs/RepoApi.md#list_repository) | **GET** /users/{owner}/repos | list repository in specific owner
*RepoApi* | [**list_repository_of_authenticated_user_double_quote**](docs/RepoApi.md#list_repository_of_authenticated_user_double_quote) | **GET** /users/repos | list repository
*RepoApi* | [**update_repository**](docs/RepoApi.md#update_repository) | **POST** /repos/{owner}/{repository} | update repository
*ReposApi* | [**get_archive**](docs/ReposApi.md#get_archive) | **GET** /repos/{owner}/{repository}/archive | get repo files archive
*TagsApi* | [**create_tag**](docs/TagsApi.md#create_tag) | **POST** /repos/{owner}/{repository}/tag | create tag
*TagsApi* | [**delete_tag**](docs/TagsApi.md#delete_tag) | **DELETE** /repos/{owner}/{repository}/tag | delete tag
*TagsApi* | [**get_tag**](docs/TagsApi.md#get_tag) | **GET** /repos/{owner}/{repository}/tag | get tag
*TagsApi* | [**list_tags**](docs/TagsApi.md#list_tags) | **GET** /repos/{owner}/{repository}/tags | list tags
*WipApi* | [**commit_wip**](docs/WipApi.md#commit_wip) | **POST** /wip/{owner}/{repository}/commit | commit working in process to branch
*WipApi* | [**delete_wip**](docs/WipApi.md#delete_wip) | **DELETE** /wip/{owner}/{repository} | remove working in process
*WipApi* | [**get_wip**](docs/WipApi.md#get_wip) | **GET** /wip/{owner}/{repository} | get working in process
*WipApi* | [**get_wip_changes**](docs/WipApi.md#get_wip_changes) | **GET** /wip/{owner}/{repository}/changes | get working in process changes
*WipApi* | [**list_wip**](docs/WipApi.md#list_wip) | **GET** /wip/{owner}/{repository}/list | list wip in specific project and user
*WipApi* | [**revert_wip_changes**](docs/WipApi.md#revert_wip_changes) | **POST** /wip/{owner}/{repository}/revert | revert changes in working in process, empty path will revert all
*WipApi* | [**update_wip**](docs/WipApi.md#update_wip) | **POST** /wip/{owner}/{repository} | update wip


## Documentation For Models

 - [Aksk](docs/Aksk.md)
 - [AkskList](docs/AkskList.md)
 - [ArchiveType](docs/ArchiveType.md)
 - [AuthenticationToken](docs/AuthenticationToken.md)
 - [Blob](docs/Blob.md)
 - [BlockStoreConfig](docs/BlockStoreConfig.md)
 - [BlockStoreConfigAzure](docs/BlockStoreConfigAzure.md)
 - [BlockStoreConfigGs](docs/BlockStoreConfigGs.md)
 - [BlockStoreConfigLocal](docs/BlockStoreConfigLocal.md)
 - [BlockStoreConfigS3](docs/BlockStoreConfigS3.md)
 - [Branch](docs/Branch.md)
 - [BranchCreation](docs/BranchCreation.md)
 - [BranchList](docs/BranchList.md)
 - [Change](docs/Change.md)
 - [ChangePair](docs/ChangePair.md)
 - [Commit](docs/Commit.md)
 - [CreateMergeRequest](docs/CreateMergeRequest.md)
 - [CreateRepository](docs/CreateRepository.md)
 - [Credential](docs/Credential.md)
 - [Error](docs/Error.md)
 - [FullTreeEntry](docs/FullTreeEntry.md)
 - [Group](docs/Group.md)
 - [LoginConfig](docs/LoginConfig.md)
 - [LoginRequest](docs/LoginRequest.md)
 - [Member](docs/Member.md)
 - [MergeMergeRequest](docs/MergeMergeRequest.md)
 - [MergeRequest](docs/MergeRequest.md)
 - [MergeRequestFullState](docs/MergeRequestFullState.md)
 - [MergeRequestList](docs/MergeRequestList.md)
 - [ObjectStats](docs/ObjectStats.md)
 - [ObjectStatsList](docs/ObjectStatsList.md)
 - [Pagination](docs/Pagination.md)
 - [RefType](docs/RefType.md)
 - [Repository](docs/Repository.md)
 - [RepositoryList](docs/RepositoryList.md)
 - [S3AuthInfo](docs/S3AuthInfo.md)
 - [SafeAksk](docs/SafeAksk.md)
 - [SetupState](docs/SetupState.md)
 - [Signature](docs/Signature.md)
 - [Tag](docs/Tag.md)
 - [TagCreation](docs/TagCreation.md)
 - [TagList](docs/TagList.md)
 - [TreeEntry](docs/TreeEntry.md)
 - [TreeNode](docs/TreeNode.md)
 - [UpdateMergeRequest](docs/UpdateMergeRequest.md)
 - [UpdateRepository](docs/UpdateRepository.md)
 - [UpdateWip](docs/UpdateWip.md)
 - [UserInfo](docs/UserInfo.md)
 - [UserRegisterInfo](docs/UserRegisterInfo.md)
 - [UserUpdate](docs/UserUpdate.md)
 - [VersionResult](docs/VersionResult.md)
 - [WebIdentity](docs/WebIdentity.md)
 - [Wip](docs/Wip.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


