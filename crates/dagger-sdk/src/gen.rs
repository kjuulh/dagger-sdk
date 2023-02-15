pub struct CacheId(String);
pub struct ContainerId(String);
pub struct DirectoryId(String);
pub struct FileId(String);
pub struct Platform(String);
pub struct SecretId(String);
pub struct SocketId(String);
pub struct BuildArg {
    pub name: String,
    pub value: String,
}
pub struct CacheVolume {

}

impl CacheVolume {

}
pub struct Container {

}

pub struct ContainerBuildOpts {
    pub context: DirectoryID,
    pub dockerfile: String,
    pub build_args: Vec<BuildArg>,
    pub target: String,
}
pub struct ContainerDirectoryOpts {
    pub path: String,
}
pub struct ContainerEnvVariableOpts {
    pub name: String,
}
pub struct ContainerExecOpts {
    pub args: Vec<String>,
    pub stdin: String,
    pub redirect_stdout: String,
    pub redirect_stderr: String,
    pub experimental_privileged_nesting: bool,
}
pub struct ContainerExportOpts {
    pub path: String,
    pub platform_variants: Vec<ContainerID>,
}
pub struct ContainerFileOpts {
    pub path: String,
}
pub struct ContainerFromOpts {
    pub address: String,
}
pub struct ContainerLabelOpts {
    pub name: String,
}
pub struct ContainerPipelineOpts {
    pub name: String,
    pub description: String,
}
pub struct ContainerPublishOpts {
    pub address: String,
    pub platform_variants: Vec<ContainerID>,
}
pub struct ContainerWithDefaultArgsOpts {
    pub args: Vec<String>,
}
pub struct ContainerWithDirectoryOpts {
    pub path: String,
    pub directory: DirectoryID,
    pub exclude: Vec<String>,
    pub include: Vec<String>,
}
pub struct ContainerWithEntrypointOpts {
    pub args: Vec<String>,
}
pub struct ContainerWithEnvVariableOpts {
    pub name: String,
    pub value: String,
}
pub struct ContainerWithExecOpts {
    pub args: Vec<String>,
    pub stdin: String,
    pub redirect_stdout: String,
    pub redirect_stderr: String,
    pub experimental_privileged_nesting: bool,
}
pub struct ContainerWithFsOpts {
    pub id: DirectoryID,
}
pub struct ContainerWithFileOpts {
    pub path: String,
    pub source: FileID,
    pub permissions: isize,
}
pub struct ContainerWithLabelOpts {
    pub name: String,
    pub value: String,
}
pub struct ContainerWithMountedCacheOpts {
    pub path: String,
    pub cache: CacheID,
    pub source: DirectoryID,
}
pub struct ContainerWithMountedDirectoryOpts {
    pub path: String,
    pub source: DirectoryID,
}
pub struct ContainerWithMountedFileOpts {
    pub path: String,
    pub source: FileID,
}
pub struct ContainerWithMountedSecretOpts {
    pub path: String,
    pub source: SecretID,
}
pub struct ContainerWithMountedTempOpts {
    pub path: String,
}
pub struct ContainerWithNewFileOpts {
    pub path: String,
    pub contents: String,
    pub permissions: isize,
}
pub struct ContainerWithRootfsOpts {
    pub id: DirectoryID,
}
pub struct ContainerWithSecretVariableOpts {
    pub name: String,
    pub secret: SecretID,
}
pub struct ContainerWithUnixSocketOpts {
    pub path: String,
    pub source: SocketID,
}
pub struct ContainerWithUserOpts {
    pub name: String,
}
pub struct ContainerWithWorkdirOpts {
    pub path: String,
}
pub struct ContainerWithoutEnvVariableOpts {
    pub name: String,
}
pub struct ContainerWithoutLabelOpts {
    pub name: String,
}
pub struct ContainerWithoutMountOpts {
    pub path: String,
}
pub struct ContainerWithoutUnixSocketOpts {
    pub path: String,
}

impl Container {

}
pub struct Directory {

}

pub struct DirectoryDiffOpts {
    pub other: DirectoryID,
}
pub struct DirectoryDirectoryOpts {
    pub path: String,
}
pub struct DirectoryDockerBuildOpts {
    pub dockerfile: String,
    pub platform: Platform,
    pub build_args: Vec<BuildArg>,
    pub target: String,
}
pub struct DirectoryEntriesOpts {
    pub path: String,
}
pub struct DirectoryExportOpts {
    pub path: String,
}
pub struct DirectoryFileOpts {
    pub path: String,
}
pub struct DirectoryLoadProjectOpts {
    pub config_path: String,
}
pub struct DirectoryPipelineOpts {
    pub name: String,
    pub description: String,
}
pub struct DirectoryWithDirectoryOpts {
    pub path: String,
    pub directory: DirectoryID,
    pub exclude: Vec<String>,
    pub include: Vec<String>,
}
pub struct DirectoryWithFileOpts {
    pub path: String,
    pub source: FileID,
    pub permissions: isize,
}
pub struct DirectoryWithNewDirectoryOpts {
    pub path: String,
    pub permissions: isize,
}
pub struct DirectoryWithNewFileOpts {
    pub path: String,
    pub contents: String,
    pub permissions: isize,
}
pub struct DirectoryWithTimestampsOpts {
    pub timestamp: isize,
}
pub struct DirectoryWithoutDirectoryOpts {
    pub path: String,
}
pub struct DirectoryWithoutFileOpts {
    pub path: String,
}

impl Directory {

}
pub struct EnvVariable {

}

impl EnvVariable {

}
pub struct File {

}

pub struct FileExportOpts {
    pub path: String,
}
pub struct FileWithTimestampsOpts {
    pub timestamp: isize,
}

impl File {

}
pub struct GitRef {

}

pub struct GitRefTreeOpts {
    pub ssh_known_hosts: String,
    pub ssh_auth_socket: SocketID,
}

impl GitRef {

}
pub struct GitRepository {

}

pub struct GitRepositoryBranchOpts {
    pub name: String,
}
pub struct GitRepositoryCommitOpts {
    pub id: String,
}
pub struct GitRepositoryTagOpts {
    pub name: String,
}

impl GitRepository {

}
pub struct Host {

}

pub struct HostDirectoryOpts {
    pub path: String,
    pub exclude: Vec<String>,
    pub include: Vec<String>,
}
pub struct HostEnvVariableOpts {
    pub name: String,
}
pub struct HostUnixSocketOpts {
    pub path: String,
}
pub struct HostWorkdirOpts {
    pub exclude: Vec<String>,
    pub include: Vec<String>,
}

impl Host {

}
pub struct HostVariable {

}

impl HostVariable {

}
pub struct Label {

}

impl Label {

}
pub struct Project {

}

impl Project {

}
pub struct Query {

}

pub struct QueryCacheVolumeOpts {
    pub key: String,
}
pub struct QueryContainerOpts {
    pub id: ContainerID,
    pub platform: Platform,
}
pub struct QueryDirectoryOpts {
    pub id: DirectoryID,
}
pub struct QueryFileOpts {
    pub id: FileID,
}
pub struct QueryGitOpts {
    pub url: String,
    pub keep_git_dir: bool,
}
pub struct QueryHttpOpts {
    pub url: String,
}
pub struct QueryPipelineOpts {
    pub name: String,
    pub description: String,
}
pub struct QueryProjectOpts {
    pub name: String,
}
pub struct QuerySecretOpts {
    pub id: SecretID,
}
pub struct QuerySocketOpts {
    pub id: SocketID,
}

impl Query {

}
pub struct Secret {

}

impl Secret {

}
pub struct Socket {

}

impl Socket {

}
