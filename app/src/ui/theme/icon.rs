//! The icons: Lucide's, exactly those the React app imported from
//! `lucide-react` (1.66), embedded as the SVGs under `app/assets/icons`
//! (ISC licence beside them) and drawn through [`icon`] alone.
//!
//! A variant is named after Lucide's file, so the names the React app used
//! for the same drawing read as: `Loader2Icon` → [`Icon::LoaderCircle`],
//! `CheckCircle2Icon` → [`Icon::CircleCheck`], `ArrowUpCircleIcon` →
//! [`Icon::CircleArrowUp`], `MoreHorizontalIcon` → [`Icon::Ellipsis`],
//! `Trash2Icon` → [`Icon::Trash`], `HistoryIcon` → [`Icon::RotateCcwClock`],
//! `ArchiveRestoreIcon` → [`Icon::ArchiveArrowUp`], `FileCode2Icon` →
//! [`Icon::FileCodeCorner`], `ShieldQuestionIcon` →
//! [`Icon::ShieldQuestionMark`].

use iced::widget::svg::{Handle, Svg};

use super::{metrics, style::svg, Theme};

/// Declares [`Icon`] with, for each variant, the file it embeds: a variant
/// without its SVG does not compile.
macro_rules! icons {
    ($($variant:ident => $file:literal,)+) => {
        /// An icon of the set.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Icon {
            $($variant,)+
        }

        impl Icon {
            /// Every icon, in the set's order.
            pub const ALL: &'static [Self] = &[$(Self::$variant,)+];

            /// Lucide's name for it (`circle-check`).
            pub const fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $file,)+
                }
            }

            /// The SVG's source.
            pub const fn bytes(self) -> &'static [u8] {
                match self {
                    $(Self::$variant => {
                        include_bytes!(concat!("../../../assets/icons/", $file, ".svg"))
                    })+
                }
            }
        }
    };
}

icons! {
    Activity => "activity",
    Archive => "archive",
    ArchiveArrowUp => "archive-arrow-up",
    ArrowDown => "arrow-down",
    ArrowDownToLine => "arrow-down-to-line",
    ArrowLeft => "arrow-left",
    ArrowRight => "arrow-right",
    ArrowRightLeft => "arrow-right-left",
    ArrowUp => "arrow-up",
    BadgeCheck => "badge-check",
    Blocks => "blocks",
    BookOpen => "book-open",
    Box => "box",
    Boxes => "boxes",
    Briefcase => "briefcase",
    BrushCleaning => "brush-cleaning",
    Camera => "camera",
    Check => "check",
    CheckCheck => "check-check",
    ChevronDown => "chevron-down",
    ChevronLeft => "chevron-left",
    ChevronRight => "chevron-right",
    ChevronUp => "chevron-up",
    CircleAlert => "circle-alert",
    CircleArrowUp => "circle-arrow-up",
    CircleCheck => "circle-check",
    CircleSlash => "circle-slash",
    CircleX => "circle-x",
    CloudDownload => "cloud-download",
    Container => "container",
    Copy => "copy",
    Cpu => "cpu",
    Database => "database",
    Disc => "disc",
    Download => "download",
    Ellipsis => "ellipsis",
    EllipsisVertical => "ellipsis-vertical",
    Eraser => "eraser",
    ExternalLink => "external-link",
    Eye => "eye",
    EyeOff => "eye-off",
    File => "file",
    FileArchive => "file-archive",
    FileCodeCorner => "file-code-corner",
    FileCog => "file-cog",
    FileKey => "file-key",
    FileUp => "file-up",
    Folder => "folder",
    FolderKanban => "folder-kanban",
    FolderOpen => "folder-open",
    FolderTree => "folder-tree",
    Gauge => "gauge",
    GitCommitHorizontal => "git-commit-horizontal",
    Group => "group",
    Hammer => "hammer",
    HardDrive => "hard-drive",
    ImageDown => "image-down",
    Info => "info",
    KeyRound => "key-round",
    Keyboard => "keyboard",
    Layers => "layers",
    LayoutDashboard => "layout-dashboard",
    LayoutGrid => "layout-grid",
    Link => "link",
    ListChecks => "list-checks",
    LoaderCircle => "loader-circle",
    Lock => "lock",
    LogIn => "log-in",
    LogOut => "log-out",
    MemoryStick => "memory-stick",
    Monitor => "monitor",
    Moon => "moon",
    Network => "network",
    OctagonX => "octagon-x",
    Paperclip => "paperclip",
    Pause => "pause",
    Pencil => "pencil",
    Play => "play",
    Plug => "plug",
    PlugZap => "plug-zap",
    Plus => "plus",
    PowerOff => "power-off",
    RefreshCw => "refresh-cw",
    Rocket => "rocket",
    RotateCcw => "rotate-ccw",
    RotateCcwClock => "rotate-ccw-clock",
    RotateCw => "rotate-cw",
    Save => "save",
    Scale => "scale",
    Scaling => "scaling",
    ScrollText => "scroll-text",
    Search => "search",
    Server => "server",
    ServerCog => "server-cog",
    ServerOff => "server-off",
    Settings => "settings",
    ShieldAlert => "shield-alert",
    ShieldCheck => "shield-check",
    ShieldOff => "shield-off",
    ShieldQuestionMark => "shield-question-mark",
    ShipWheel => "ship-wheel",
    Skull => "skull",
    SlidersHorizontal => "sliders-horizontal",
    Square => "square",
    SquareTerminal => "square-terminal",
    Star => "star",
    Sun => "sun",
    Table => "table",
    Tag => "tag",
    Trash => "trash",
    TriangleAlert => "triangle-alert",
    Truck => "truck",
    Undo2 => "undo-2",
    Unplug => "unplug",
    Upload => "upload",
    Waypoints => "waypoints",
    X => "x",
    Zap => "zap",
}

/// `icon` at Lucide's `size-4`, in the theme's text colour. A widget cannot
/// see the colour of the text around it, so where that is not the page's
/// (on a primary button, in muted or red text) say so:
/// `icon(Icon::Trash).style(theme::svg::destructive)`; resize it with
/// `.width(..).height(..)` and the sizes of `metrics::icon`.
pub fn icon(icon: Icon) -> Svg<'static, Theme> {
    Svg::new(Handle::from_memory(icon.bytes()))
        .width(metrics::icon::MD)
        .height(metrics::icon::MD)
        .style(svg::foreground)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fs, path::Path};

    use super::Icon;

    #[test]
    fn every_icon_is_an_embedded_svg_that_parses() {
        for icon in Icon::ALL {
            let tree = usvg::Tree::from_data(icon.bytes(), &usvg::Options::default())
                .unwrap_or_else(|e| panic!("{}: {e}", icon.name()));
            assert_eq!(tree.size().width(), 24.0, "{}", icon.name());
            assert!(tree.root().has_children(), "{} draws nothing", icon.name());
        }
    }

    /// The folder holds the set and nothing else, so an icon dropped from
    /// the enum does not stay in the binary's sources.
    #[test]
    fn the_folder_and_the_enum_list_the_same_icons() {
        let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/icons");
        let files: HashSet<String> = fs::read_dir(folder)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().is_some_and(|e| e == "svg"))
            .map(|path| path.file_stem().unwrap().to_string_lossy().into_owned())
            .collect();
        let named: HashSet<String> = Icon::ALL.iter().map(|i| i.name().to_owned()).collect();
        assert_eq!(files, named);
        assert_eq!(named.len(), Icon::ALL.len(), "two variants share a file");
    }
}
