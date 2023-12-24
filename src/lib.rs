//! dsync library
//!
//! The dsync library allows creating a custom binary for dsync
//!
//! ## Features
//!
//! - `async`: enable support for [diesel_async](https://github.com/weiznich/diesel_async)
//! - `tsync`: enable support for [tsync](https://github.com/Wulf/tsync)
//! - `backtrace`: enable attaching backtraces to dsync errors
//!
//! default features: `tsync`, `backtrace`

mod code;
pub mod error;
mod file;
mod parser;

use error::IOErrorToError;
pub use error::{Error, Result};
use file::MarkedFile;
use heck::ToSnakeCase;
use parser::ParsedTableMacro;
pub use parser::FILE_SIGNATURE;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::{Path, PathBuf};

/// Available options for string types
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum StringType {
    /// Use `String`
    #[default]
    String,
    /// Use `&str`
    Str,
    /// Use `Cow<str>`
    Cow,
}

impl StringType {
    /// Get the current [StringType] as a rust type string
    pub fn as_str(&self) -> &'static str {
        match self {
            StringType::String => "String",
            StringType::Str => "&'a str",
            StringType::Cow => "Cow<'a, str>",
        }
    }

    /// Get the lifetime used for the current [StringType]
    pub fn get_lifetime(&self) -> &'static str {
        match self {
            StringType::String => "",
            StringType::Str => "'a",
            StringType::Cow => "'a",
        }
    }
}

/// Available options for bytes types
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BytesType {
    /// Use `Vec<u8>`
    #[default]
    Vec,
    /// Use `&[u8]`
    Slice,
    /// Use `Cow<[u8]>`
    Cow,
}

impl BytesType {
    /// Get the current [BytesType] as a rust type string
    pub fn as_str(&self) -> &'static str {
        match self {
            BytesType::Vec => "Vec<u8>",
            BytesType::Slice => "&'a [u8]",
            BytesType::Cow => "Cow<'a, [u8]>",
        }
    }

    /// Get the lifetime used for the current [BytesType]
    pub fn get_lifetime(&self) -> &'static str {
        match self {
            BytesType::Vec => "",
            BytesType::Slice => "'a",
            BytesType::Cow => "'a",
        }
    }
}

/// Options for a individual table
#[derive(Debug, Clone)]
pub struct TableOptions<'a> {
    /// Ignore a specific table
    ignore: Option<bool>,
    /// Names used for autogenerated columns which are NOT primary keys (for example: `created_at`, `updated_at`, etc.).
    autogenerated_columns: Option<Vec<&'a str>>,

    #[cfg(feature = "tsync")]
    /// Adds #[tsync] attribute to structs (see <https://github.com/Wulf/tsync>)
    tsync: Option<bool>,

    #[cfg(feature = "async")]
    /// Uses diesel_async for generated functions (see <https://github.com/weiznich/diesel_async>)
    use_async: Option<bool>,

    /// Generates `serde::Serialize` and `serde::Deserialize` derive implementations
    use_serde: bool,

    /// Generates the CRUD functions for generated models
    fns: bool,

    /// Determines which string type to use for Create* structs
    create_str_type: StringType,

    /// Determines which string type to use for Update* structs
    update_str_type: StringType,

    /// Determines which bytes type to use for Create* structs
    create_bytes_type: BytesType,

    /// Determines which bytes type to use for Update* structs
    update_bytes_type: BytesType,

    /// Only Generate a single model file instead of a directory with "mod.rs" and "generated.rs"
    single_model_file: bool,

    /// Indiciates this table is meant to be read-only (dont generate Update & Create structs)
    read_only: bool,
}

impl<'a> TableOptions<'a> {
    pub fn get_ignore(&self) -> bool {
        self.ignore.unwrap_or_default()
    }

    #[cfg(feature = "tsync")]
    pub fn get_tsync(&self) -> bool {
        self.tsync.unwrap_or_default()
    }

    #[cfg(feature = "async")]
    pub fn get_async(&self) -> bool {
        self.use_async.unwrap_or_default()
    }

    pub fn get_serde(&self) -> bool {
        self.use_serde
    }

    pub fn get_fns(&self) -> bool {
        self.fns
    }

    pub fn get_create_str_type(&self) -> StringType {
        self.create_str_type
    }

    pub fn get_update_str_type(&self) -> StringType {
        self.update_str_type
    }

    pub fn get_create_bytes_type(&self) -> BytesType {
        self.create_bytes_type
    }

    pub fn get_update_bytes_type(&self) -> BytesType {
        self.update_bytes_type
    }

    pub fn get_autogenerated_columns(&self) -> &[&'_ str] {
        self.autogenerated_columns.as_deref().unwrap_or_default()
    }

    pub fn get_readonly(&self) -> bool {
        self.read_only
    }

    pub fn ignore(self) -> Self {
        Self {
            ignore: Some(true),
            ..self
        }
    }

    #[cfg(feature = "tsync")]
    pub fn tsync(self) -> Self {
        Self {
            tsync: Some(true),
            ..self
        }
    }

    #[cfg(feature = "async")]
    pub fn use_async(self) -> Self {
        Self {
            use_async: Some(true),
            ..self
        }
    }

    pub fn disable_serde(self) -> Self {
        Self {
            use_serde: false,
            ..self
        }
    }

    pub fn disable_fns(self) -> Self {
        Self { fns: false, ..self }
    }

    pub fn single_model_file(self) -> Self {
        Self {
            single_model_file: true,
            ..self
        }
    }

    pub fn autogenerated_columns(self, cols: Vec<&'a str>) -> Self {
        Self {
            autogenerated_columns: Some(cols),
            ..self
        }
    }

    pub fn create_str_type(self, type_: StringType) -> Self {
        Self {
            create_str_type: type_,
            ..self
        }
    }

    pub fn update_str_type(self, type_: StringType) -> Self {
        Self {
            update_str_type: type_,
            ..self
        }
    }

    pub fn create_bytes_type(self, type_: BytesType) -> Self {
        Self {
            create_bytes_type: type_,
            ..self
        }
    }

    pub fn update_bytes_type(self, type_: BytesType) -> Self {
        Self {
            update_bytes_type: type_,
            ..self
        }
    }

    pub fn set_read_only(&mut self, value: bool) {
        self.read_only = value;
    }

    /// Fills any `None` properties with values from another TableConfig
    pub fn apply_defaults(&self, other: &TableOptions<'a>) -> Self {
        Self {
            ignore: self.ignore.or(other.ignore),
            #[cfg(feature = "tsync")]
            tsync: self.tsync.or(other.tsync),
            #[cfg(feature = "async")]
            use_async: self.use_async.or(other.use_async),
            autogenerated_columns: self
                .autogenerated_columns
                .clone()
                .or_else(|| other.autogenerated_columns.clone()),

            use_serde: self.use_serde || other.use_serde,
            fns: self.fns || other.fns,
            create_str_type: other.create_str_type,
            update_str_type: other.update_str_type,
            create_bytes_type: other.create_bytes_type,
            update_bytes_type: other.update_bytes_type,
            single_model_file: self.single_model_file || other.single_model_file,
            read_only: self.read_only || other.read_only,
        }
    }
}

impl<'a> Default for TableOptions<'a> {
    fn default() -> Self {
        Self {
            ignore: Default::default(),
            autogenerated_columns: Default::default(),
            #[cfg(feature = "tsync")]
            tsync: Default::default(),
            #[cfg(feature = "async")]
            use_async: Default::default(),
            use_serde: true,
            fns: true,
            create_str_type: Default::default(),
            update_str_type: Default::default(),
            create_bytes_type: Default::default(),
            update_bytes_type: Default::default(),
            single_model_file: false,
            read_only: false,
        }
    }
}

/// Global config, not table specific
#[derive(Debug, Clone)]
pub struct GenerationConfig<'a> {
    /// Specific Table options for a given table
    pub table_options: HashMap<&'a str, TableOptions<'a>>,
    /// Default table options, used when not in `table_options`
    pub default_table_options: TableOptions<'a>,
    /// Connection type to insert
    ///
    /// For example:
    /// - `diesel::pg::PgConnection`
    /// - `diesel::sqlite::SqliteConnection`
    /// - `diesel::mysql::MysqlConnection`
    /// - `diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>`
    /// - or, your custom diesel connection type (struct which implements `diesel::connection::Connection`)
    pub connection_type: String,
    /// Diesel schema import path
    ///
    /// by default `crate::schema::`
    pub schema_path: String,
    /// Dsync model import path
    ///
    /// by default `crate::models::`
    pub model_path: String,
    /// Generate common structs only once in a "common.rs" file
    pub once_common_structs: bool,
    /// Generate the "ConnectionType" type only once in a "common.rs" file
    pub once_connection_type: bool,
    /// Prefixes to treat tables as readonly
    pub readonly_prefixes: Vec<String>,
    /// Suffixes to treat tables as readonly
    pub readonly_suffixes: Vec<String>,
}

impl GenerationConfig<'_> {
    pub fn table(&self, name: &str) -> TableOptions<'_> {
        let table = self
            .table_options
            .get(name)
            .unwrap_or(&self.default_table_options);

        let mut table = table.apply_defaults(&self.default_table_options);

        if self.readonly_prefixes.iter().any(|v| name.starts_with(v))
            || self.readonly_suffixes.iter().any(|v| name.ends_with(v))
        {
            table.set_read_only(true);
        }

        table
    }
}

/// Generate a model for the given schema contents
///
/// Model is returned and not saved to disk yet
pub fn generate_code(
    diesel_schema_file_contents: &str,
    config: &GenerationConfig,
) -> Result<Vec<ParsedTableMacro>> {
    parser::parse_and_generate_code(diesel_schema_file_contents, config)
}

/// Status indicating what happened to a file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileChangeStatus {
    /// Status for unchanged file contents
    Unchanged,
    /// Status for modified file contents
    Modified,
    /// Status if the file has been deleted
    Deleted,
}

impl Display for FileChangeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FileChangeStatus::Unchanged => "Unchanged",
                FileChangeStatus::Modified => "Modified",
                FileChangeStatus::Deleted => "Deleted",
            }
        )
    }
}

/// Status indicating what happened to a specific file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileChange {
    /// File in question
    pub file: PathBuf,
    /// Status of the file
    pub status: FileChangeStatus,
}

impl FileChange {
    pub fn new<P: AsRef<std::path::Path>>(path: P, status: FileChangeStatus) -> Self {
        Self {
            file: path.as_ref().to_owned(),
            status,
        }
    }
}

// easily create a [FileChange] from a [MarkedFile]
impl From<&MarkedFile> for FileChange {
    fn from(value: &MarkedFile) -> Self {
        if value.is_modified() {
            Self::new(&value.path, FileChangeStatus::Modified)
        } else {
            Self::new(&value.path, FileChangeStatus::Unchanged)
        }
    }
}

/// Helper function for consistent table module name generation
/// this is used for the rust module path name and for the filename
///
/// input: "tableA", output -> "table_a"
fn get_table_module_name(table_name: &str) -> String {
    table_name.to_snake_case().to_lowercase()
}

/// Generate all Models for a given diesel schema file
///
/// Models are saved to disk
pub fn generate_files(
    input_diesel_schema_file: &Path,
    output_models_dir: &Path,
    config: GenerationConfig,
) -> Result<Vec<FileChange>> {
    let generated = generate_code(
        &std::fs::read_to_string(input_diesel_schema_file)
            .attach_path_err(input_diesel_schema_file)?,
        &config,
    )?;

    if !output_models_dir.exists() {
        std::fs::create_dir(output_models_dir).attach_path_err(output_models_dir)?;
    } else if !output_models_dir.is_dir() {
        return Err(Error::not_a_directory(
            "Expected output argument to be a directory or non-existent.",
            output_models_dir,
        ));
    }

    // using generated len, because that is very likely the amount (at least) for files
    let mut file_changes = Vec::with_capacity(generated.len());

    // check that the mod.rs file exists
    let mut mod_rs = MarkedFile::new(output_models_dir.join("mod.rs"))?;

    if config.once_common_structs || config.once_connection_type {
        let mut common_file = MarkedFile::new(output_models_dir.join("common.rs"))?;
        common_file.ensure_file_signature()?;
        common_file.change_file_contents({
            let mut tmp = format!("{FILE_SIGNATURE}\n");
            if config.once_common_structs {
                tmp.push_str(&code::generate_common_structs(
                    &config.default_table_options,
                ));
            }
            if config.once_connection_type {
                tmp.push('\n');
                tmp.push_str(&code::generate_connection_type(&config));

                // add ending new-line, this should not cause duplicate new-lines because this only gets run if any of the options is set
                // this will need to be refactored if there should ever be more options using common_file
                tmp.push('\n');
            }

            tmp
        });
        common_file.write()?;
        file_changes.push(FileChange::from(&common_file));

        mod_rs.ensure_mod_stmt("common");
    }

    // pass 1: add code for new tables
    for table in generated.iter() {
        if config.once_common_structs && table.name == "common" {
            return Err(Error::other("Cannot have a table named \"common\" while having option \"once_common_structs\" enabled"));
        }
        let table_name = table.name.to_string();
        let table_filename = get_table_module_name(&table_name);
        let table_config = config.table(&table_name);
        let table_dir = if table_config.single_model_file {
            output_models_dir.to_owned()
        } else {
            output_models_dir.join(&table_filename)
        };

        if !table_dir.exists() {
            std::fs::create_dir(&table_dir).attach_path_err(&table_dir)?;
        }

        if !table_dir.is_dir() {
            return Err(Error::not_a_directory("Expected a directory", table_dir));
        }

        let table_file_name = if table_config.single_model_file {
            let mut table_name = table_name;
            table_name.push_str(".rs");
            table_name
        } else {
            "generated.rs".into()
        };

        let mut table_generated_rs = MarkedFile::new(table_dir.join(table_file_name))?;
        let mut table_mod_rs = MarkedFile::new(table_dir.join("mod.rs"))?;

        table_generated_rs.ensure_file_signature()?;
        table_generated_rs.change_file_contents(table.generated_code.clone());
        table_generated_rs.write()?;

        file_changes.push(FileChange::from(&table_generated_rs));

        if !table_config.single_model_file {
            table_mod_rs.ensure_mod_stmt("generated");
            table_mod_rs.ensure_use_stmt("generated::*");
            table_mod_rs.write()?;
            file_changes.push(FileChange::from(&table_mod_rs));
        }

        mod_rs.ensure_mod_stmt(&table_filename);
    }

    // pass 2: delete code for removed tables
    for item in std::fs::read_dir(output_models_dir).attach_path_err(output_models_dir)? {
        // TODO: this does not work with "single-model-file"
        let item = item.attach_path_err(output_models_dir)?;

        // check if item is a directory
        let file_type = item
            .file_type()
            .attach_path_msg(item.path(), "Could not determine type of file")?;
        if !file_type.is_dir() {
            continue;
        }

        // check if it's a generated file
        let generated_rs_path = item.path().join("generated.rs");
        if !generated_rs_path.exists()
            || !generated_rs_path.is_file()
            || !MarkedFile::new(generated_rs_path.clone())?.has_file_signature()
        {
            continue;
        }

        // okay, it's generated, but we need to check if it's for a deleted table
        let file_name = item.file_name();
        let associated_table_name = file_name.to_str().ok_or(Error::other(format!(
            "Could not determine name of file '{:#?}'",
            item.path()
        )))?;
        let found = generated.iter().find(|g| {
            get_table_module_name(&g.name.to_string()).eq_ignore_ascii_case(associated_table_name)
        });
        if found.is_some() {
            continue;
        }

        // this table was deleted, let's delete the generated code
        std::fs::remove_file(&generated_rs_path).attach_path_err(&generated_rs_path)?;
        file_changes.push(FileChange::new(
            &generated_rs_path,
            FileChangeStatus::Deleted,
        ));

        // remove the mod.rs file if there isn't anything left in there except the use stmt
        let table_mod_rs_path = item.path().join("mod.rs");
        if table_mod_rs_path.exists() {
            let mut table_mod_rs = MarkedFile::new(table_mod_rs_path)?;

            table_mod_rs.remove_mod_stmt("generated");
            table_mod_rs.remove_use_stmt("generated::*");
            table_mod_rs.write()?;

            if table_mod_rs.get_file_contents().trim().is_empty() {
                let table_mod_rs = table_mod_rs.delete()?;
                file_changes.push(FileChange::new(table_mod_rs, FileChangeStatus::Deleted));
            } else {
                table_mod_rs.write()?; // write the changes we made above
                file_changes.push(FileChange::from(&table_mod_rs));
            }
        }

        // delete the table dir if there's nothing else in there
        let is_empty = item
            .path()
            .read_dir()
            .attach_path_err(item.path())?
            .next()
            .is_none();
        if is_empty {
            std::fs::remove_dir(item.path()).attach_path_err(item.path())?;
        }

        // remove the module from the main mod_rs file
        mod_rs.remove_mod_stmt(associated_table_name);
    }

    mod_rs.write()?;
    file_changes.push(FileChange::from(&mod_rs));

    Ok(file_changes)
}
