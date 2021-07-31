use crate::utils::{format_datetime, format_duration};
use crate::wrapper::service::ServiceName;
use crate::wrapper::tag::Tag;
use chrono::{Datelike, Duration};
use mime::Mime;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct TagBuilder {
    negated: bool,
    name: String,
    namespace: Option<String>,
}

impl TagBuilder {
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            negated: false,
            name: name.to_string(),
            namespace: None,
        }
    }

    /// Set a namespace for the tag
    pub fn namespace<S: ToString>(mut self, namespace: S) -> Self {
        self.namespace = Some(namespace.to_string());

        self
    }

    /// Converts the builder into a system tag builder
    pub fn system(self) -> SystemTagBuilder {
        SystemTagBuilder {
            negated: false,
            name: self.name,
        }
    }

    /// Negates the tag.
    /// if it has already been negated it will be positive again
    pub fn negate(mut self) -> Self {
        self.negated = !self.negated;

        self
    }

    pub fn build(self) -> Tag {
        Tag {
            negated: self.negated,
            name: self.name,
            namespace: self.namespace,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SystemTagBuilder {
    name: String,
    negated: bool,
}

impl SystemTagBuilder {
    pub fn new() -> SystemTagBuilder {
        SystemTagBuilder {
            name: String::new(),
            negated: false,
        }
    }

    pub fn build(self) -> Tag {
        Tag {
            negated: self.negated,
            name: self.name,
            namespace: Some(String::from("system")),
        }
    }

    /// Negates the tag.
    /// if it has already been negated it will be positive again
    pub fn negate(mut self) -> Self {
        self.negated = !self.negated;

        self
    }

    /// All files stored in the client
    pub fn everything(self) -> Self {
        self.change_name("everything")
    }

    /// Files stored in the inbox
    pub fn inbox(self) -> Self {
        self.change_name("inbox")
    }

    /// Archived files
    pub fn archive(self) -> Self {
        self.change_name("archive")
    }

    /// Files that have a duration (e.g. videos)
    pub fn has_duration(self) -> Self {
        self.change_name("has duration")
    }

    /// Files that don't have a duration
    pub fn no_duration(self) -> Self {
        self.change_name("no duration")
    }

    /// Files with a specific duration
    pub fn duration(self, comparator: Comparator, value: u64, unit: DurationUnit) -> Self {
        self.change_name(format!("duration {} {} {}", comparator, value, unit))
    }

    /// Files that have the best quality in their duplicate group
    pub fn best_duplicate_quality(self) -> Self {
        self.change_name("best quality of group")
    }

    /// Files that don't have the best quality in their duplicate group
    pub fn not_best_duplicate_quality(self) -> Self {
        self.change_name("isn't best quality of group")
    }

    /// Files with audio
    pub fn has_audio(self) -> Self {
        self.change_name("has audio")
    }

    /// Files without audio
    pub fn no_audio(self) -> Self {
        self.change_name("no audio")
    }

    /// Files with tags
    pub fn has_tags(self) -> Self {
        self.change_name("has tags")
    }

    /// Files without tags
    pub fn no_tags(self) -> Self {
        self.change_name("no tags")
    }

    /// Untagged files
    pub fn untagged(self) -> Self {
        self.change_name("untagged")
    }

    /// Files with a specific number of tags
    pub fn number_of_tags(self, comparator: Comparator, value: u64) -> Self {
        self.change_name(format!("number of tags {} {}", comparator, value))
    }

    /// Files with a specific height
    pub fn height(self, comparator: Comparator, value: u64) -> Self {
        self.change_name(format!("height {} {}", comparator, value))
    }

    /// Files with a specific width
    pub fn width(self, comparator: Comparator, value: u64) -> Self {
        self.change_name(format!("width {} {}", comparator, value))
    }

    /// Files with a specific filesize
    pub fn filesize(self, comparator: Comparator, value: u64, unit: FileSizeUnit) -> Self {
        self.change_name(format!("filesize {} {} {}", comparator, value, unit))
    }

    /// Files that are similar to a list of other files with a specific [hamming distance](https://en.wikipedia.org/wiki/Hamming_distance)
    pub fn similar_to(self, hashes: Vec<String>, distance: u32) -> Self {
        self.change_name(format!(
            "similar to {} with distance {}",
            hashes.join(", "),
            distance
        ))
    }

    /// Limit the number of returned files
    pub fn limit(self, value: u64) -> Self {
        self.change_name(format!("limit = {}", value))
    }

    /// Files with a specific mimetype
    pub fn filetype(self, mimes: Vec<Mime>) -> Self {
        self.change_name(format!(
            "filetype = {}",
            mimes
                .into_iter()
                .map(|m| m.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        ))
    }

    /// Files with a specific hash
    pub fn hash(self, hashes: Vec<String>) -> Self {
        self.change_name(format!("hash = {}", hashes.join(" ")))
    }

    /// Files that have been modified before / after / at / around a specific date and time
    pub fn date_modified<D: Datelike>(self, comparator: Comparator, datetime: D) -> Self {
        self.change_name(format!(
            "modified date {} {}",
            comparator,
            format_datetime(datetime)
        ))
    }

    /// Files with a specific import time
    pub fn time_imported<D: Datelike>(self, comparator: Comparator, datetime: D) -> Self {
        self.change_name(format!(
            "time imported {} {}",
            comparator,
            format_datetime(datetime)
        ))
    }

    /// Files that are in a file service or pending to it
    pub fn file_service(
        self,
        comparator: IsComparator,
        cur_pen: CurrentlyOrPending,
        service: ServiceName,
    ) -> Self {
        self.change_name(format!(
            "file service {} {} {}",
            comparator, cur_pen, service
        ))
    }

    /// Files that have a specific number of relationships
    pub fn number_of_relationships(
        self,
        comparator: Comparator,
        value: u64,
        relationship: FileRelationshipType,
    ) -> Self {
        self.change_name(format!(
            "num file relationships {} {} {}",
            comparator, value, relationship
        ))
    }

    /// Files with a specific aspect ratio
    pub fn ratio(self, wte: WiderTallerEqual, value: (u64, u64)) -> Self {
        self.change_name(format!("ratio {} {}:{}", wte, value.0, value.1))
    }

    /// Files with a specific number of pixels
    pub fn number_of_pixels(self, comparator: Comparator, value: u64, unit: PixelUnit) -> Self {
        self.change_name(format!("num pixels {} {} {}", comparator, value, unit))
    }

    /// Files that have been viewed a specific number of times
    pub fn views(self, view_type: ViewType, comparator: Comparator, value: u64) -> Self {
        self.change_name(format!("{} views {} {}", view_type, comparator, value))
    }

    /// Files that have been viewed for a specific number of times
    pub fn viewtime(self, view_type: ViewType, comparator: Comparator, duration: Duration) -> Self {
        self.change_name(format!(
            "{} viewtime {} {}",
            view_type,
            comparator,
            format_duration(duration)
        ))
    }

    /// Files that have associated urls that match a defined regex
    pub fn has_url_matching_regex<S: Display>(self, regex: S) -> Self {
        self.change_name(format!("has url matching regex {}", regex))
    }

    /// Files that don't have an url that matches a defined regex
    pub fn does_not_have_url_matching_regex<S: Display>(self, regex: S) -> Self {
        self.change_name(format!("does not have url matching regex {}", regex))
    }

    /// Files that have an url that matches a class (e.g. 'safebooru file page')
    pub fn has_url_with_class<S: Display>(self, class: S) -> Self {
        self.change_name(format!("has url with class {}", class))
    }

    /// Files that don't have an url that matches a class (e.g. 'safebooru file page')
    pub fn does_not_have_url_with_class<S: Display>(self, class: S) -> Self {
        self.change_name(format!("does not have url with class {}", class))
    }

    /// Converts a tag namespace (e.g. 'page') into a number and compares it
    pub fn tag_namespace_as_number<S: Display>(
        self,
        namespace: S,
        comparator: Comparator,
        value: u64,
    ) -> Self {
        self.change_name(format!(
            "tag as number {} {} {}",
            namespace, comparator, value
        ))
    }

    fn change_name<S: ToString>(mut self, value: S) -> Self {
        self.name = value.to_string();

        self
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum Comparator {
    /// rhs > lhs
    Greater,
    /// rhs < lhs
    Less,
    /// rhs == lhs
    Equal,
    /// If the rhs is in a +-15% range of the lhs
    Approximate,
}

impl Display for Comparator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Comparator::Greater => ">",
            Comparator::Less => "<",
            Comparator::Equal => "=",
            Comparator::Approximate => "~=",
        };
        symbol.fmt(f)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum FileSizeUnit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
}

impl Display for FileSizeUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            FileSizeUnit::Bytes => "B",
            FileSizeUnit::Kilobytes => "KB",
            FileSizeUnit::Megabytes => "MB",
            FileSizeUnit::Gigabytes => "GB",
        };
        name.fmt(f)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum DurationUnit {
    Hours,
    Minutes,
    Seconds,
    Milliseconds,
}

impl Display for DurationUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            DurationUnit::Hours => "hours",
            DurationUnit::Minutes => "minutes",
            DurationUnit::Seconds => "seconds",
            DurationUnit::Milliseconds => "milliseconds",
        };
        name.fmt(f)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum IsComparator {
    Is,
    IsNot,
}

impl Display for IsComparator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            IsComparator::Is => "is",
            IsComparator::IsNot => "is not",
        };
        name.fmt(f)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum CurrentlyOrPending {
    CurrentlyIn,
    PendingTo,
}

impl Display for CurrentlyOrPending {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CurrentlyOrPending::CurrentlyIn => "currently in",
            CurrentlyOrPending::PendingTo => "pending to",
        };
        name.fmt(f)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum WiderTallerEqual {
    Wider,
    Taller,
    Equal,
}

impl Display for WiderTallerEqual {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            WiderTallerEqual::Wider => "wider than",
            WiderTallerEqual::Taller => "taller than",
            WiderTallerEqual::Equal => "is",
        };
        name.fmt(f)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum PixelUnit {
    Pixels,
    Kilopixels,
    Megapixels,
}

impl Display for PixelUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            PixelUnit::Pixels => "pixels",
            PixelUnit::Kilopixels => "kilopixels",
            PixelUnit::Megapixels => "megapixels",
        };
        name.fmt(f)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum ViewType {
    Media,
    Preview,
    All,
}

impl Display for ViewType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ViewType::Media => "media",
            ViewType::Preview => "preview",
            ViewType::All => "all",
        };
        name.fmt(f)
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum FileRelationshipType {
    Alternates,
    FalsePositives,
    Duplicates,
    PotentialDuplicates,
}

impl Display for FileRelationshipType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            FileRelationshipType::Alternates => "alternates",
            FileRelationshipType::FalsePositives => "false positives",
            FileRelationshipType::Duplicates => "duplicates",
            FileRelationshipType::PotentialDuplicates => "potential duplicates",
        };
        name.fmt(f)
    }
}
