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
pub struct CacheVolume {}

impl CacheVolume {
    pub fn id(&self) -> CacheId {
        todo!()
    }
}
pub struct Container {}

pub struct ContainerBuildOpts {
    pub dockerfile: Option<String>,
    pub build_args: Option<Vec<BuildArg>>,
    pub target: Option<String>,
}
pub struct ContainerExecOpts {
    pub args: Option<Vec<String>>,
    pub stdin: Option<String>,
    pub redirect_stdout: Option<String>,
    pub redirect_stderr: Option<String>,
    pub experimental_privileged_nesting: Option<bool>,
}
pub struct ContainerExportOpts {
    pub platform_variants: Option<Vec<ContainerId>>,
}
pub struct ContainerPipelineOpts {
    pub description: Option<String>,
}
pub struct ContainerPublishOpts {
    pub platform_variants: Option<Vec<ContainerId>>,
}
pub struct ContainerWithDefaultArgsOpts {
    pub args: Option<Vec<String>>,
}
pub struct ContainerWithDirectoryOpts {
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
}
pub struct ContainerWithExecOpts {
    pub stdin: Option<String>,
    pub redirect_stdout: Option<String>,
    pub redirect_stderr: Option<String>,
    pub experimental_privileged_nesting: Option<bool>,
}
pub struct ContainerWithFileOpts {
    pub permissions: Option<isize>,
}
pub struct ContainerWithMountedCacheOpts {
    pub source: Option<DirectoryId>,
}
pub struct ContainerWithNewFileOpts {
    pub contents: Option<String>,
    pub permissions: Option<isize>,
}

impl Container {
    pub fn build(&self, context: DirectoryId, opts: Option<ContainerBuildOpts>) -> Container {
        todo!()
    }
    pub fn default_args(&self) -> Vec<String> {
        todo!()
    }
    pub fn directory(&self, path: String) -> Directory {
        todo!()
    }
    pub fn entrypoint(&self) -> Vec<String> {
        todo!()
    }
    pub fn env_variable(&self, name: String) -> String {
        todo!()
    }
    pub fn env_variables(&self) -> Vec<EnvVariable> {
        todo!()
    }
    pub fn exec(&self, opts: Option<ContainerExecOpts>) -> Container {
        todo!()
    }
    pub fn exit_code(&self) -> isize {
        todo!()
    }
    pub fn export(&self, path: String, opts: Option<ContainerExportOpts>) -> bool {
        todo!()
    }
    pub fn file(&self, path: String) -> File {
        todo!()
    }
    pub fn from(&self, address: String) -> Container {
        todo!()
    }
    pub fn fs(&self) -> Directory {
        todo!()
    }
    pub fn id(&self) -> ContainerId {
        todo!()
    }
    pub fn label(&self, name: String) -> String {
        todo!()
    }
    pub fn labels(&self) -> Vec<Label> {
        todo!()
    }
    pub fn mounts(&self) -> Vec<String> {
        todo!()
    }
    pub fn pipeline(&self, name: String, opts: Option<ContainerPipelineOpts>) -> Container {
        todo!()
    }
    pub fn platform(&self) -> Platform {
        todo!()
    }
    pub fn publish(&self, address: String, opts: Option<ContainerPublishOpts>) -> String {
        todo!()
    }
    pub fn rootfs(&self) -> Directory {
        todo!()
    }
    pub fn stderr(&self) -> String {
        todo!()
    }
    pub fn stdout(&self) -> String {
        todo!()
    }
    pub fn user(&self) -> String {
        todo!()
    }
    pub fn with_default_args(&self, opts: Option<ContainerWithDefaultArgsOpts>) -> Container {
        todo!()
    }
    pub fn with_directory(
        &self,
        path: String,
        directory: DirectoryId,
        opts: Option<ContainerWithDirectoryOpts>,
    ) -> Container {
        todo!()
    }
    pub fn with_entrypoint(&self, args: Vec<String>) -> Container {
        todo!()
    }
    pub fn with_env_variable(&self, name: String, value: String) -> Container {
        todo!()
    }
    pub fn with_exec(&self, args: Vec<String>, opts: Option<ContainerWithExecOpts>) -> Container {
        todo!()
    }
    pub fn with_fs(&self, id: DirectoryId) -> Container {
        todo!()
    }
    pub fn with_file(
        &self,
        path: String,
        source: FileId,
        opts: Option<ContainerWithFileOpts>,
    ) -> Container {
        todo!()
    }
    pub fn with_label(&self, name: String, value: String) -> Container {
        todo!()
    }
    pub fn with_mounted_cache(
        &self,
        path: String,
        cache: CacheId,
        opts: Option<ContainerWithMountedCacheOpts>,
    ) -> Container {
        todo!()
    }
    pub fn with_mounted_directory(&self, path: String, source: DirectoryId) -> Container {
        todo!()
    }
    pub fn with_mounted_file(&self, path: String, source: FileId) -> Container {
        todo!()
    }
    pub fn with_mounted_secret(&self, path: String, source: SecretId) -> Container {
        todo!()
    }
    pub fn with_mounted_temp(&self, path: String) -> Container {
        todo!()
    }
    pub fn with_new_file(&self, path: String, opts: Option<ContainerWithNewFileOpts>) -> Container {
        todo!()
    }
    pub fn with_rootfs(&self, id: DirectoryId) -> Container {
        todo!()
    }
    pub fn with_secret_variable(&self, name: String, secret: SecretId) -> Container {
        todo!()
    }
    pub fn with_unix_socket(&self, path: String, source: SocketId) -> Container {
        todo!()
    }
    pub fn with_user(&self, name: String) -> Container {
        todo!()
    }
    pub fn with_workdir(&self, path: String) -> Container {
        todo!()
    }
    pub fn without_env_variable(&self, name: String) -> Container {
        todo!()
    }
    pub fn without_label(&self, name: String) -> Container {
        todo!()
    }
    pub fn without_mount(&self, path: String) -> Container {
        todo!()
    }
    pub fn without_unix_socket(&self, path: String) -> Container {
        todo!()
    }
    pub fn workdir(&self) -> String {
        todo!()
    }
}
pub struct Directory {}

pub struct DirectoryDockerBuildOpts {
    pub dockerfile: Option<String>,
    pub platform: Option<Platform>,
    pub build_args: Option<Vec<BuildArg>>,
    pub target: Option<String>,
}
pub struct DirectoryEntriesOpts {
    pub path: Option<String>,
}
pub struct DirectoryPipelineOpts {
    pub description: Option<String>,
}
pub struct DirectoryWithDirectoryOpts {
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
}
pub struct DirectoryWithFileOpts {
    pub permissions: Option<isize>,
}
pub struct DirectoryWithNewDirectoryOpts {
    pub permissions: Option<isize>,
}
pub struct DirectoryWithNewFileOpts {
    pub permissions: Option<isize>,
}

impl Directory {
    pub fn diff(&self, other: DirectoryId) -> Directory {
        todo!()
    }
    pub fn directory(&self, path: String) -> Directory {
        todo!()
    }
    pub fn docker_build(&self, opts: Option<DirectoryDockerBuildOpts>) -> Container {
        todo!()
    }
    pub fn entries(&self, opts: Option<DirectoryEntriesOpts>) -> Vec<String> {
        todo!()
    }
    pub fn export(&self, path: String) -> bool {
        todo!()
    }
    pub fn file(&self, path: String) -> File {
        todo!()
    }
    pub fn id(&self) -> DirectoryId {
        todo!()
    }
    pub fn load_project(&self, config_path: String) -> Project {
        todo!()
    }
    pub fn pipeline(&self, name: String, opts: Option<DirectoryPipelineOpts>) -> Directory {
        todo!()
    }
    pub fn with_directory(
        &self,
        path: String,
        directory: DirectoryId,
        opts: Option<DirectoryWithDirectoryOpts>,
    ) -> Directory {
        todo!()
    }
    pub fn with_file(
        &self,
        path: String,
        source: FileId,
        opts: Option<DirectoryWithFileOpts>,
    ) -> Directory {
        todo!()
    }
    pub fn with_new_directory(
        &self,
        path: String,
        opts: Option<DirectoryWithNewDirectoryOpts>,
    ) -> Directory {
        todo!()
    }
    pub fn with_new_file(
        &self,
        path: String,
        contents: String,
        opts: Option<DirectoryWithNewFileOpts>,
    ) -> Directory {
        todo!()
    }
    pub fn with_timestamps(&self, timestamp: isize) -> Directory {
        todo!()
    }
    pub fn without_directory(&self, path: String) -> Directory {
        todo!()
    }
    pub fn without_file(&self, path: String) -> Directory {
        todo!()
    }
}
pub struct EnvVariable {}

impl EnvVariable {
    pub fn name(&self) -> String {
        todo!()
    }
    pub fn value(&self) -> String {
        todo!()
    }
}
pub struct File {}

impl File {
    pub fn contents(&self) -> String {
        todo!()
    }
    pub fn export(&self, path: String) -> bool {
        todo!()
    }
    pub fn id(&self) -> FileId {
        todo!()
    }
    pub fn secret(&self) -> Secret {
        todo!()
    }
    pub fn size(&self) -> isize {
        todo!()
    }
    pub fn with_timestamps(&self, timestamp: isize) -> File {
        todo!()
    }
}
pub struct GitRef {}

pub struct GitRefTreeOpts {
    pub ssh_known_hosts: Option<String>,
    pub ssh_auth_socket: Option<SocketId>,
}

impl GitRef {
    pub fn digest(&self) -> String {
        todo!()
    }
    pub fn tree(&self, opts: Option<GitRefTreeOpts>) -> Directory {
        todo!()
    }
}
pub struct GitRepository {}

impl GitRepository {
    pub fn branch(&self, name: String) -> GitRef {
        todo!()
    }
    pub fn branches(&self) -> Vec<String> {
        todo!()
    }
    pub fn commit(&self, id: String) -> GitRef {
        todo!()
    }
    pub fn tag(&self, name: String) -> GitRef {
        todo!()
    }
    pub fn tags(&self) -> Vec<String> {
        todo!()
    }
}
pub struct Host {}

pub struct HostDirectoryOpts {
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
}
pub struct HostWorkdirOpts {
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
}

impl Host {
    pub fn directory(&self, path: String, opts: Option<HostDirectoryOpts>) -> Directory {
        todo!()
    }
    pub fn env_variable(&self, name: String) -> HostVariable {
        todo!()
    }
    pub fn unix_socket(&self, path: String) -> Socket {
        todo!()
    }
    pub fn workdir(&self, opts: Option<HostWorkdirOpts>) -> Directory {
        todo!()
    }
}
pub struct HostVariable {}

impl HostVariable {
    pub fn secret(&self) -> Secret {
        todo!()
    }
    pub fn value(&self) -> String {
        todo!()
    }
}
pub struct Label {}

impl Label {
    pub fn name(&self) -> String {
        todo!()
    }
    pub fn value(&self) -> String {
        todo!()
    }
}
pub struct Project {}

impl Project {
    pub fn extensions(&self) -> Vec<Project> {
        todo!()
    }
    pub fn generated_code(&self) -> Directory {
        todo!()
    }
    pub fn install(&self) -> bool {
        todo!()
    }
    pub fn name(&self) -> String {
        todo!()
    }
    pub fn schema(&self) -> String {
        todo!()
    }
    pub fn sdk(&self) -> String {
        todo!()
    }
}
pub struct Query {}

pub struct QueryContainerOpts {
    pub id: Option<ContainerId>,
    pub platform: Option<Platform>,
}
pub struct QueryDirectoryOpts {
    pub id: Option<DirectoryId>,
}
pub struct QueryGitOpts {
    pub keep_git_dir: Option<bool>,
}
pub struct QueryPipelineOpts {
    pub description: Option<String>,
}
pub struct QuerySocketOpts {
    pub id: Option<SocketId>,
}

impl Query {
    pub fn cache_volume(&self, key: String) -> CacheVolume {
        todo!()
    }
    pub fn container(&self, opts: Option<QueryContainerOpts>) -> Container {
        todo!()
    }
    pub fn default_platform(&self) -> Platform {
        todo!()
    }
    pub fn directory(&self, opts: Option<QueryDirectoryOpts>) -> Directory {
        todo!()
    }
    pub fn file(&self, id: FileId) -> File {
        todo!()
    }
    pub fn git(&self, url: String, opts: Option<QueryGitOpts>) -> GitRepository {
        todo!()
    }
    pub fn host(&self) -> Host {
        todo!()
    }
    pub fn http(&self, url: String) -> File {
        todo!()
    }
    pub fn pipeline(&self, name: String, opts: Option<QueryPipelineOpts>) -> Query {
        todo!()
    }
    pub fn project(&self, name: String) -> Project {
        todo!()
    }
    pub fn secret(&self, id: SecretId) -> Secret {
        todo!()
    }
    pub fn socket(&self, opts: Option<QuerySocketOpts>) -> Socket {
        todo!()
    }
}
pub struct Secret {}

impl Secret {
    pub fn id(&self) -> SecretId {
        todo!()
    }
    pub fn plaintext(&self) -> String {
        todo!()
    }
}
pub struct Socket {}

impl Socket {
    pub fn id(&self) -> SocketId {
        todo!()
    }
}
