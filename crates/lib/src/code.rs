use heck::{ToPascalCase, ToSnakeCase};

use crate::parser::{ParsedTableMacro, FILE_SIGNATURE};
use crate::{GenerationConfig, Result, TableOptions};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StructType {
    /// Type for the main struct, which can be queried and has all properties
    Read,
    // this struct type maps directly to a database row
    /// Type for the Update struct, which has all properties wrapped in [Option]
    Update,
    /// Type for the Create struct, which only has all properties that are not autogenerated (like rowid)
    Create,
}

impl StructType {
    /// Get the prefix for the current struct type
    pub fn prefix(&self) -> &'static str {
        match self {
            StructType::Read => "",
            StructType::Update => "Update",
            StructType::Create => "Create",
        }
    }

    /// Get the suffix for the current struct type
    pub fn suffix(&self) -> &'static str {
        match self {
            StructType::Read => "",
            StructType::Update => "",
            StructType::Create => "",
        }
    }

    /// Get the formatted struct with prefix and suffix applied
    pub fn format(&self, name: &'_ str) -> String {
        format!(
            "{struct_prefix}{struct_name}{struct_suffix}",
            struct_prefix = self.prefix(),
            struct_name = name,
            struct_suffix = self.suffix()
        )
    }
}

#[derive(Debug)]
struct Struct<'a> {
    /// Struct name
    identifier: String,
    /// Type of the struct
    ty: StructType,
    /// Parsed table from diesel schema
    table: &'a ParsedTableMacro,
    /// Options for the current table
    opts: TableOptions<'a>,
    /// Generator wide config (original config)
    config: &'a GenerationConfig<'a>,
    /// Storage for the once rendered code
    rendered_code: Option<String>,
    /// Storage for if the current struct has any fields
    has_fields: Option<bool>, // note: this is only correctly set after a call to render() which gets called in Struct::new()
}

#[derive(Debug, Clone)]
pub struct StructField {
    /// Name for the field
    pub name: String,
    /// Type of the final field
    pub base_type: String,

    pub is_optional: bool,
}

#[allow(non_upper_case_globals)]
impl<'a> Struct<'a> {
    pub fn new(
        ty: StructType,
        table: &'a ParsedTableMacro,
        config: &'a GenerationConfig<'_>,
    ) -> Self {
        let mut obj = Self {
            identifier: ty.format(table.struct_name.as_str()),
            opts: config.table(&table.name.to_string()),
            table,
            ty,
            config,
            rendered_code: None,
            has_fields: None,
        };
        obj.render();
        obj
    }

    pub fn code(&self) -> &str {
        self.rendered_code.as_deref().unwrap_or_default()
    }

    pub fn has_fields(&self) -> bool {
        self.has_fields.unwrap()
    }

    fn attr_tsync(&self) -> &'static str {
        #[cfg(feature = "tsync")]
        match self.opts.get_tsync() {
            true => "#[tsync::tsync]\n",
            false => "",
        }
        #[cfg(not(feature = "tsync"))]
        ""
    }

    const DERIVES_DEFAULT: &[&'static str] = &["Debug", "Clone"];
    const DERIVE_Queryable: &str = "Queryable";
    const DERIVE_Insertable: &str = "Insertable";
    const DERIVE_Selectable: &str = "Selectable";
    const DERIVE_Associations: &str = "Associations";
    const DERIVE_Identifiable: &str = "Identifiable";
    const DERIVE_AsChangeset: &str = "AsChangeset";
    const DERIVE_Serde_Serialize: &str = "Serialize";
    const DERIVE_Serde_Deserialize: &str = "Deserialize";

    fn attr_derive(&self) -> String {
        let mut derives: Vec<&str> = Vec::from(Self::DERIVES_DEFAULT);

        if self.config.default_table_options.get_serde() {
            derives.push(Self::DERIVE_Serde_Serialize);
            derives.push(Self::DERIVE_Serde_Deserialize);
        }

        if !self.opts.get_only_necessary_derives() || (self.opts.get_only_necessary_derives() && self.ty == StructType::Read) {
            derives.push(Self::DERIVE_Queryable);
        }
        
        if !self.opts.get_only_necessary_derives() || (self.opts.get_only_necessary_derives() && self.ty == StructType::Create) {
            derives.push(Self::DERIVE_Insertable);
        }

        if self.ty == StructType::Read {
            derives.push(Self::DERIVE_Selectable);

            if !self.table.foreign_keys.is_empty() {
                derives.push(Self::DERIVE_Identifiable);
            }

            if !self.table.foreign_keys.is_empty() {
                derives.push(Self::DERIVE_Associations);
            }
        }

        if !self
            .fields()
            .iter()
            .all(|f| self.table.primary_key_column_names().contains(&f.name))
        {
            derives.push(Self::DERIVE_AsChangeset)
        }

        let derives = derives.join(", ");

        format!("#[derive({})]", derives)
    }

    fn fields(&self) -> Vec<StructField> {
        self.table
            .columns
            .iter()
            .filter(|c| {
                let is_autogenerated = self
                    .opts
                    .autogenerated_columns
                    .as_deref()
                    .unwrap_or_default()
                    .contains(&c.name.to_string().as_str());

                match self.ty {
                    StructType::Read => true,
                    StructType::Update => {
                        let is_pk = self.table.primary_key_columns.contains(&c.name);

                        !is_pk
                    }
                    StructType::Create => !is_autogenerated,
                }
            })
            .map(|c| {
                let name = c.name.to_string();
                let base_type = if c.is_nullable {
                    format!("Option<{}>", c.ty)
                } else if c.is_unsigned {
                    c.ty.replace('i', "u")
                } else {
                    c.ty.clone()
                };
                let mut is_optional = false;

                let is_pk = self
                    .table
                    .primary_key_columns
                    .iter()
                    .any(|pk| pk.to_string().eq(name.as_str()));
                let is_autogenerated = self
                    .opts
                    .autogenerated_columns
                    .as_deref()
                    .unwrap_or_default()
                    .contains(&c.name.to_string().as_str());
                // let is_fk = table.foreign_keys.iter().any(|fk| fk.1.to_string().eq(field_name.as_str()));

                match self.ty {
                    StructType::Read => {}
                    StructType::Update => {
                        // all non-key fields should be optional in Form structs (to allow partial updates)
                        is_optional = !is_pk || is_autogenerated;
                    }
                    StructType::Create => {}
                }

                StructField {
                    name,
                    base_type,
                    is_optional,
                }
            })
            .collect()
    }

    fn render(&mut self) {
        let ty = self.ty;
        let table = &self.table;
        let _opts = self.config.table(table.name.to_string().as_str());

        let primary_keys: Vec<String> = table.primary_key_column_names();

        let belongs_to = table
            .foreign_keys
            .iter()
            .map(|fk| {
                format!(
                    ", belongs_to({foreign_table_name}, foreign_key={join_column})",
                    foreign_table_name = fk.0.to_string().to_pascal_case(),
                    join_column = fk.1
                )
            })
            .collect::<Vec<String>>()
            .join(" ");

        let struct_code = format!(
            "{tsync_attr}{derive_attr}
#[diesel(table_name={table_name}{primary_key}{belongs_to})]
pub struct {struct_name} {{
$COLUMNS$
}}\n",
            tsync_attr = self.attr_tsync(),
            derive_attr = self.attr_derive(),
            table_name = table.name,
            struct_name = ty.format(table.struct_name.as_str()),
            primary_key = if ty != StructType::Read {
                "".to_string()
            } else {
                format!(", primary_key({})", primary_keys.join(","))
            },
            belongs_to = if ty != StructType::Read {
                "".to_string()
            } else {
                belongs_to
            }
        );

        let fields = self.fields();
        let mut lines = vec![];
        for f in fields.iter() {
            let field_name = &f.name;
            let field_type = if f.is_optional {
                format!("Option<{}>", f.base_type)
            } else {
                f.base_type.clone()
            };

            lines.push(format!(r#"    pub {field_name}: {field_type},"#));
        }

        if fields.is_empty() {
            self.has_fields = Some(false);
            self.rendered_code = Some("".to_string());
        } else {
            self.has_fields = Some(true);
            self.rendered_code = Some(struct_code.replace("$COLUMNS$", &lines.join("\n")));
        }
    }
}

fn build_table_fns(
    table: &ParsedTableMacro,
    config: &GenerationConfig,
    create_struct: Struct,
    update_struct: Struct,
) -> Result<String> {
    let table_options = config.table(&table.name.to_string());

    let primary_column_name_and_type: Vec<(String, String)> = table
        .primary_key_columns
        .iter()
        .map(|pk| {
            let col = table
                .columns
                .iter()
                .find(|it| it.name.to_string().eq(pk.to_string().as_str()))
                .expect("Primary key column doesn't exist in table");

            (col.name.to_string(), col.ty.to_string())
        })
        .collect();

    let item_id_params = primary_column_name_and_type
        .iter()
        .map(|name_and_type| {
            format!(
                "param_{name}: {ty}",
                name = name_and_type.0,
                ty = name_and_type.1
            )
        })
        .collect::<Vec<String>>()
        .join(", ");
    let item_id_filters = primary_column_name_and_type
        .iter()
        .map(|name_and_type| {
            format!(
                "filter({name}.eq(param_{name}))",
                name = name_and_type.0.to_string()
            )
        })
        .collect::<Vec<String>>()
        .join(".");

    // template variables
    let table_name = table.name.to_string();
    #[cfg(feature = "tsync")]
    let tsync = match table_options.get_tsync() {
        true => "#[tsync::tsync]",
        false => "",
    };
    #[cfg(not(feature = "tsync"))]
    let tsync = "";
    #[cfg(feature = "async")]
    let async_keyword = if table_options.get_async() {
        " async"
    } else {
        ""
    };
    #[cfg(not(feature = "async"))]
    let async_keyword = "";
    #[cfg(feature = "async")]
    let await_keyword = if table_options.get_async() {
        ".await"
    } else {
        ""
    };
    #[cfg(not(feature = "async"))]
    let await_keyword = "";
    let struct_name = &table.struct_name;
    let schema_path = &config.schema_path;
    let create_struct_identifier = &create_struct.identifier;
    let update_struct_identifier = &update_struct.identifier;
    let item_id_params = item_id_params;
    let item_id_filters = item_id_filters;

    let mut buffer = String::new();

    buffer.push_str(&format!(
        r##"{tsync}
#[derive(Debug, {serde_derive})]
pub struct PaginationResult<T> {{
    pub items: Vec<T>,
    pub total_items: i64,
    /// 0-based index
    pub page: i64,
    pub page_size: i64,
    pub num_pages: i64,
}}
"##,
        serde_derive = if config.default_table_options.get_serde() {
            Struct::DERIVE_Serde_Serialize
        } else {
            ""
        }
    ));

    buffer.push_str(&format!(
        r##"
impl {struct_name} {{
"##
    ));

    if create_struct.has_fields() {
        buffer.push_str(&format!(
            r##"
    pub{async_keyword} fn create(db: &mut Connection, item: &{create_struct_identifier}) -> QueryResult<Self> {{
        use {schema_path}{table_name}::dsl::*;

        insert_into({table_name}).values(item).get_result::<Self>(db){await_keyword}
    }}
"##
        ));
    } else {
        buffer.push_str(&format!(
            r##"
    pub{async_keyword} fn create(db: &mut Connection) -> QueryResult<Self> {{
        use {schema_path}{table_name}::dsl::*;

        insert_into({table_name}).default_values().get_result::<Self>(db){await_keyword}
    }}
"##
        ));
    }

    buffer.push_str(&format!(
        r##"
    pub{async_keyword} fn read(db: &mut Connection, {item_id_params}) -> QueryResult<Self> {{
        use {schema_path}{table_name}::dsl::*;

        {table_name}.{item_id_filters}.first::<Self>(db){await_keyword}
    }}
"##
    ));

    buffer.push_str(&format!(r##"
    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub{async_keyword} fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {{
        use {schema_path}{table_name}::dsl::*;

        let page_size = if page_size < 1 {{ 1 }} else {{ page_size }};
        let total_items = {table_name}.count().get_result(db){await_keyword}?;
        let items = {table_name}.limit(page_size).offset(page * page_size).load::<Self>(db){await_keyword}?;

        Ok(PaginationResult {{
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        }})
    }}
"##));

    // TODO: If primary key columns are attached to the form struct (not optionally)
    // then don't require item_id_params (otherwise it'll be duplicated)

    // if has_update_struct {
    if update_struct.has_fields() {
        // It's possible we have a form struct with all primary keys (for example, for a join table).
        // In this scenario, we also have to check whether there are any updatable columns for which
        // we should generate an update() method.

        buffer.push_str(&format!(r##"
    pub{async_keyword} fn update(db: &mut Connection, {item_id_params}, item: &{update_struct_identifier}) -> QueryResult<Self> {{
        use {schema_path}{table_name}::dsl::*;

        diesel::update({table_name}.{item_id_filters}).set(item).get_result(db){await_keyword}
    }}
"##));
    }

    buffer.push_str(&format!(
        r##"
    pub{async_keyword} fn delete(db: &mut Connection, {item_id_params}) -> QueryResult<usize> {{
        use {schema_path}{table_name}::dsl::*;

        diesel::delete({table_name}.{item_id_filters}).execute(db){await_keyword}
    }}
"##
    ));

    buffer.push_str(
        r##"
}"##,
    );

    Ok(buffer)
}

/// Generate all the imports that are required
fn build_imports(table: &ParsedTableMacro, config: &GenerationConfig) -> String {
    #[cfg(feature = "async")]
    let table_options = config.table(&table.name.to_string());
    let belongs_imports = table
        .foreign_keys
        .iter()
        .map(|fk| {
            format!(
                "use {model_path}{foreign_table_name_model}::{singular_struct_name};",
                foreign_table_name_model = fk.0.to_string().to_snake_case().to_lowercase(),
                singular_struct_name = fk.0.to_string().to_pascal_case(),
                model_path = config.model_path
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    #[cfg(feature = "async")]
    let async_imports = if table_options.get_async() {
        "\nuse diesel_async::RunQueryDsl;"
    } else {
        ""
    };
    #[cfg(not(feature = "async"))]
    let async_imports = "";
    let serde_imports = if config.default_table_options.get_serde() {
        "use serde::{Deserialize, Serialize};"
    } else {
        ""
    };

    let mut schema_path = config.schema_path.clone();
    schema_path.push('*');
    format!(
        "use crate::diesel::*;
use {schema_path};
use diesel::QueryResult;
{serde_imports}{async_imports}
{belongs_imports}

type Connection = {connection_type};\n",
        connection_type = config.connection_type,
        belongs_imports = belongs_imports,
        async_imports = async_imports,
        serde_imports = serde_imports,
        schema_path = schema_path
    )
}

/// Generate full file for for a given diesel table
pub fn generate_for_table(table: ParsedTableMacro, config: &GenerationConfig) -> Result<String> {
    // first, we generate struct code
    let read_struct = Struct::new(StructType::Read, &table, config);
    let update_struct = Struct::new(StructType::Update, &table, config);
    let create_struct = Struct::new(StructType::Create, &table, config);

    let mut structs = String::new();
    structs.push_str(read_struct.code());
    structs.push('\n');
    structs.push_str(create_struct.code());
    structs.push('\n');
    structs.push_str(update_struct.code());

    let functions = build_table_fns(&table, config, create_struct, update_struct)?;
    let imports = build_imports(&table, config);

    Ok(format!(
        "{FILE_SIGNATURE}\n\n{imports}\n{structs}\n{functions}"
    ))
}
