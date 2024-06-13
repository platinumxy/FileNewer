use crate::file_manager::FileInfo;

#[derive(PartialEq)]
#[derive(Clone)]
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

    pub(crate) fn sort(&self, file_info: &mut Vec<FileInfo> ) {
        if self.sort_by == SortBy::Nan { return; }

        for i in 1..file_info.len(){
            let mut j = i;
            while j > 0 && self.should_swap(&file_info[j-1], &file_info[j]) {
                file_info.swap(j, j-1);
                j = j-1
            }
        }
    }
    fn should_swap(&self, a:&FileInfo, b:&FileInfo) -> bool{
        match self.sort_by{
            SortBy::Name => { self.cmp(&a.file_name, &b.file_name) }
            SortBy::Ext => { self.cmp(&a.file_ext, &b.file_ext) }
            SortBy::CreateDate => { self.cmp(&a.creation_time, &b.creation_time) }
            SortBy::ModDate => { self.cmp(&a.last_modification, &b.last_modification) }
            SortBy::Size => { self.cmp(&a.file_size, &b.file_size) }
            SortBy::Type => { self.cmp(&a.single_char_desc(), &b.single_char_desc()) }
            SortBy::ViewDate => { self.cmp(&a.last_access, &b.last_access)}
            SortBy::Nan => {false}
        }
    }

    fn cmp<T: PartialOrd>(&self, a:&T, b:&T) -> bool{
        if self.filter_dec { a < b } else { a > b }
    }

}

