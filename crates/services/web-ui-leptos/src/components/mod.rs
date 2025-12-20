//! UI components.

pub mod alert;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod compose_message;
pub mod cva;
pub mod dialog;
pub mod filter_bar;
pub mod inline_message_detail;
pub mod input;
pub mod label;
pub mod layout;
pub mod mark_read_button;
pub mod message_detail_header;
pub mod overseer_composer;
pub mod pagination;
pub mod progress;
pub mod project_card;
pub mod select;
pub mod separator;
pub mod skeleton;
pub mod spinner;
pub mod split_view;
pub mod switch;
pub mod tabs;
pub mod textarea;
pub mod toast;
pub mod tooltip;

pub use alert::{Alert, AlertDescription, AlertTitle, AlertVariant};
pub use avatar::{AgentAvatar, AvatarSize};
pub use badge::{Badge, BadgeVariant};
pub use breadcrumb::{Breadcrumb, BreadcrumbItem};
pub use button::{Button, ButtonSize, ButtonVariant};
pub use card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
pub use compose_message::{ComposeMessage, ComposeProps, ReplyTo};
pub use dialog::{
    Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};

pub use filter_bar::{FilterBar, FilterState};
pub use inline_message_detail::InlineMessageDetail;
pub use input::Input;
pub use layout::Layout;
pub use mark_read_button::MarkReadButton;
pub use message_detail_header::MessageDetailHeader;
pub use overseer_composer::{OverseerComposeProps, OverseerComposer};
pub use pagination::Pagination;
pub use project_card::{ProjectCard, ProjectStatus, determine_project_status};
pub use select::{Select, SelectOption};
pub use separator::{Orientation, Separator};
pub use skeleton::{
    AttachmentCardSkeleton, AttachmentGridSkeleton, CardSkeleton, MessageDetailSkeleton,
    MessageItemSkeleton, MessageListSkeleton, Skeleton, TableRowSkeleton,
};
pub use split_view::{EmptyDetailPanel, MessageListItem, SplitViewLayout};
pub use toast::{Toast, ToastVariant, Toaster, ToasterContext, use_toaster};

// New shadcn-ui components
pub use checkbox::{Checkbox, CheckboxState};
pub use label::Label;
pub use progress::{Progress, ProgressIndeterminate};
pub use spinner::{Spinner, SpinnerSize};
pub use switch::Switch;
pub use tabs::{TabItem, Tabs, TabsContent, TabsContext, TabsList, TabsTrigger};
pub use textarea::Textarea;
pub use tooltip::{SimpleTooltip, Tooltip, TooltipSide};

// Magic UI - animated components
pub mod magic;
pub use magic::{
    AnimatedGradient, AnimatedGradientText, BlurFade, BlurFadeStagger, FadeDirection,
    FormattedNumber, GradientDirection, GridPattern, GridPatternMasked, GridType, NumberCounter,
    ShimmerBadge, ShimmerText, TypingText, TypingTextCss,
};
