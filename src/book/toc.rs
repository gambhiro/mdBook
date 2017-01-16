use book::chapter::Chapter;

/// A Table of Contents is a `Vec<TocItem>`, where an item is an enum that
/// qualifies its content.
#[derive(Debug, Clone)]
pub enum TocItem {
    Numbered(TocContent),
    Unnumbered(TocContent),
    Unlisted(TocContent),
    Spacer,
}

/// An entry in the TOC with content. Its payload is the Chapter. This struct
/// knows the section index of the entry, or contains optional sub-entries as
/// `Vec<TocItem>`.
#[derive(Debug, Clone)]
pub struct TocContent {
    pub chapter: Chapter,
    pub sub_items: Option<Vec<TocItem>>,
    /// Section indexes of the chapter
    pub section: Option<Vec<i32>>,
}

impl Default for TocContent {
    fn default() -> TocContent {
        TocContent {
            chapter: Chapter::default(),
            sub_items: None,
            section: None,
        }
    }
}

impl TocContent {

    pub fn new(chapter: Chapter) -> TocContent {
        let mut toc = TocContent::default();
        toc.chapter = chapter;
        toc
    }

    pub fn new_with_section(chapter: Chapter, section: Vec<i32>) -> TocContent {
        let mut toc = TocContent::default();
        toc.chapter = chapter;
        toc.section = Some(section);
        toc
    }

    pub fn section_as_string(&self) -> String {
        if let Some(ref sec) = self.section {
            let a = sec.iter().map(|x| x.to_string()).collect::<Vec<String>>();
            format!("{}.", a.join("."))
        } else {
            "".to_string()
        }
    }

    pub fn is_it_a_translation_of(&self, checking: &TocContent) -> bool {
        // if the user has set the same translation_id on them
        if let Some(ref a) = self.chapter.translation_id {
            if let Some(ref b) = checking.chapter.translation_id {
                if a == b {
                    return true;
                }
            }
        }

        // if src_path matches
        if let Some(ref a) = self.chapter.get_src_path() {
            if let Some(ref b) = checking.chapter.get_src_path() {
                if a == b {
                    return true;
                }
            }
        }

        false
    }

    // TODO update

    // /// This function takes a slice `&[x,y,z]` and returns the corresponding sub-chapter if it exists.
    // ///
    // /// For example: `chapter.get_sub_chapter(&[1,3])` will return the third sub-chapter of the first sub-chapter.
    // pub fn get_sub_chapter(&self, section: &[usize]) -> Option<&Chapter> {
    //     match section.len() {
    //         0 => None,
    //         1 => self.sub_chapters.get(section[0]),
    //         _ => {
    //             // The lengt of the slice is more than one, this means that we want a sub-chapter of a sub-chapter
    //             // We call `get_sub_chapter` recursively until we are deep enough and return the asked sub-chapter
    //             self.sub_chapters
    //                 .get(section[0])
    //                 .and_then(|ch| ch.get_sub_chapter(&section[1..]))
    //         },
    //     }
    // }
}

pub fn flat_toc(toc: &Vec<TocItem>) -> Vec<TocItem> {
    let mut flattened: Vec<TocItem> = vec![];
    for i in toc.iter() {
        match *i {
            TocItem::Numbered(ref x) |
            TocItem::Unnumbered(ref x) |
            TocItem::Unlisted(ref x) => {
                flattened.push(i.clone());
                if let Some(ref subs) = x.sub_items {
                    let mut a = flat_toc(subs);
                    flattened.append(&mut a);
                }
            },
            TocItem::Spacer => { flattened.push(i.clone()); },
        }
    }
    flattened
}
