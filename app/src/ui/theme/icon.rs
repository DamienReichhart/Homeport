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

use iced::{
    advanced::{
        layout, mouse, renderer,
        svg::{Renderer as _, Svg},
        widget::Tree,
        Layout, Widget,
    },
    widget::svg::{self as svg_widget, Handle},
    Length, Rectangle, Size,
};

use super::{metrics, Element, Theme};

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

type Style<'a> = Box<dyn Fn(&Theme, svg_widget::Status) -> svg_widget::Style + 'a>;

/// An icon of the set, drawn in the colour of the text around it, as
/// Lucide's `currentColor`: Paper on the primary button, red in a
/// destructive one, faded in a disabled one, without being told.
pub struct Glyph<'a> {
    handle: Handle,
    width: Length,
    height: Length,
    style: Option<Style<'a>>,
}

/// `icon` at Lucide's `size-4`, in the colour of the text around it. Give
/// it another only where it differs from that text (a muted chevron beside
/// a label): `icon(Icon::ChevronDown).style(theme::svg::muted)`; resize it
/// with `.width(..).height(..)` and the sizes of `metrics::icon`.
pub fn icon<'a>(icon: Icon) -> Glyph<'a> {
    Glyph {
        handle: Handle::from_memory(icon.bytes()),
        width: Length::Fixed(metrics::icon::MD),
        height: Length::Fixed(metrics::icon::MD),
        style: None,
    }
}

impl<'a> Glyph<'a> {
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// A colour of its own, from `theme::svg`. A style that names no
    /// colour leaves it the text's.
    #[must_use]
    pub fn style(
        mut self,
        style: impl Fn(&Theme, svg_widget::Status) -> svg_widget::Style + 'a,
    ) -> Self {
        self.style = Some(Box::new(style));
        self
    }

    /// The colour it is drawn in where the text around it is `text`.
    pub fn colour(&self, theme: &Theme, text: iced::Color) -> iced::Color {
        self.style
            .as_ref()
            .and_then(|style| style(theme, svg_widget::Status::Idle).color)
            .unwrap_or(text)
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Glyph<'_> {
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if let Some(clip) = bounds.intersection(viewport) {
            let colour = self.colour(theme, style.text_color);
            renderer.draw_svg(Svg::new(self.handle.clone()).color(colour), bounds, clip);
        }
    }
}

impl<'a, Message: 'a> From<Glyph<'a>> for Element<'a, Message> {
    fn from(glyph: Glyph<'a>) -> Self {
        Element::new(glyph)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fs, path::Path};

    use super::{icon, Icon};
    use crate::ui::theme::{self, Theme};

    #[test]
    fn an_icon_takes_the_colour_of_the_text_around_it_unless_given_one() {
        for theme in Theme::ALL {
            let tokens = theme.tokens();
            // On the primary button the text is `primary_foreground`: so
            // is the icon, which used to be drawn ink on ink.
            let around = tokens.primary_foreground;
            assert_eq!(icon(Icon::Play).colour(&theme, around), around);
            assert_ne!(icon(Icon::Play).colour(&theme, around), tokens.primary);
            let muted = icon(Icon::ChevronDown).style(theme::svg::muted);
            assert_eq!(muted.colour(&theme, around), tokens.muted_foreground);
        }
    }

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
