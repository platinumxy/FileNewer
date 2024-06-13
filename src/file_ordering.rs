#[derive(PartialEq)]
pub(crate) enum SortBy {
    Name,
    Type,
    Ext,
    CreateDate,
    ModDate,
    ViewDate,
    Size,
    Nan,
}

pub(crate) struct DisplayOptions{
    pub(crate) show_file_type:bool,
    pub(crate) show_hidden:bool,
    pub(crate) show_file_ext: bool,
    pub(crate) show_file_size: bool,
    pub(crate) show_last_acc: bool,
    pub(crate) show_last_mod: bool,
    pub(crate) show_creation: bool,
    pub(crate) sort_by: SortBy,
    pub(crate) filter_dec:bool,
}

impl DisplayOptions{
    pub(crate) fn default() -> Self{
        Self {
            show_file_type: true,
            show_hidden: false,
            show_file_ext: true,
            show_file_size: true,
            show_last_acc: false,
            show_last_mod: true,
            show_creation: true,
            sort_by: SortBy::Nan,
            filter_dec: false,
        }
    }

}
