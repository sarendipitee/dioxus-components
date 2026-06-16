use dioxus_components::avatar::{ImageAvatar, AvatarImageSize};
use dioxus_components::button::{Button, ButtonVariant};
use dioxus_components::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use dioxus_components::dropdown_menu::{DropdownMenu, DropdownMenuTrigger};
use dioxus_components::menu::{Menu, MenuItem, MenuSeparator};
use dioxus_components::separator::Separator;
use dioxus_components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel,
    SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuAction, SidebarMenuBadge,
    SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem, SidebarMenuSub,
    SidebarMenuSubButton, SidebarMenuSubItem, SidebarProvider, SidebarRail, SidebarSide,
    SidebarTrigger, SidebarVariant,
};
use dioxus_components::skeleton::Skeleton;
use dioxus_icons::lucide::{ChevronRight, Circle};
use dioxus::prelude::*;

#[css_module("/src/components/sidebar/demos/demo.css")]
struct DemoStyles;

#[derive(Clone, PartialEq)]
struct Team {
    name: &'static str,
    plan: &'static str,
}

#[derive(Clone, PartialEq)]
struct NavMainItem {
    title: &'static str,
    url: &'static str,
    is_active: bool,
    items: &'static [SubItem],
}

#[derive(Clone, PartialEq)]
struct SubItem {
    title: &'static str,
    url: &'static str,
}

#[derive(Clone, PartialEq)]
struct Project {
    name: &'static str,
    url: &'static str,
}

const TEAMS: &[Team] = &[
    Team {
        name: "Acme Inc",
        plan: "Enterprise",
    },
    Team {
        name: "Acme Corp.",
        plan: "Startup",
    },
    Team {
        name: "Evil Corp.",
        plan: "Free",
    },
];

const NAV_MAIN: &[NavMainItem] = &[
    NavMainItem {
        title: "Playground",
        url: "#",
        is_active: true,
        items: &[
            SubItem {
                title: "History",
                url: "#",
            },
            SubItem {
                title: "Starred",
                url: "#",
            },
            SubItem {
                title: "Settings",
                url: "#",
            },
        ],
    },
    NavMainItem {
        title: "Models",
        url: "#",
        is_active: false,
        items: &[
            SubItem {
                title: "Genesis",
                url: "#",
            },
            SubItem {
                title: "Explorer",
                url: "#",
            },
            SubItem {
                title: "Quantum",
                url: "#",
            },
        ],
    },
    NavMainItem {
        title: "Documentation",
        url: "#",
        is_active: false,
        items: &[
            SubItem {
                title: "Introduction",
                url: "#",
            },
            SubItem {
                title: "Get Started",
                url: "#",
            },
            SubItem {
                title: "Tutorials",
                url: "#",
            },
            SubItem {
                title: "Changelog",
                url: "#",
            },
        ],
    },
    NavMainItem {
        title: "Settings",
        url: "#",
        is_active: false,
        items: &[
            SubItem {
                title: "General",
                url: "#",
            },
            SubItem {
                title: "Team",
                url: "#",
            },
            SubItem {
                title: "Billing",
                url: "#",
            },
            SubItem {
                title: "Limits",
                url: "#",
            },
        ],
    },
];

const PROJECTS: &[Project] = &[
    Project {
        name: "Design Engineering",
        url: "#",
    },
    Project {
        name: "Sales & Marketing",
        url: "#",
    },
    Project {
        name: "Travel",
        url: "#",
    },
];

#[component]
pub fn Demo() -> Element {
    let side = use_signal(|| SidebarSide::Left);
    let collapsible = use_signal(|| SidebarCollapsible::Offcanvas);

    rsx! {
        SidebarProvider {
            Sidebar {
                variant: SidebarVariant::Floating,
                collapsible: collapsible(),
                side: side(),
                SidebarHeader {
                    TeamSwitcher { teams: TEAMS }
                }
                SidebarContent {
                    NavMain { items: NAV_MAIN }
                    NavProjects { projects: PROJECTS }
                }
                SidebarFooter { NavUser {} }
                SidebarRail {}
            }
            SidebarInset {
                header { style: "display:flex; align-items:center; justify-content:space-between; height:3.5rem; flex-shrink:0; padding:0 1rem; border-bottom:1px solid var(--surface-border); background:var(--surface-muted);",
                    div { style: "display: flex; align-items: center; gap: 0.75rem;",
                        SidebarTrigger {}
                        Separator { height: "1rem", horizontal: false }
                        span { "Sidebar Setting" }
                    }
                }
                div { style: "display:flex; flex:1; flex-direction:column; gap:1.5rem; padding:1.5rem; min-height:0; overflow-y:auto; overflow-x:hidden;",
                    DemoSettingControls { side, collapsible }
                    Skeleton { style: "height: 10rem; width: 100%; flex-shrink:0;" }
                    Skeleton { style: "height: 20rem; width: 100%; flex-shrink:0;" }
                }
            }
        }
    }
}

#[component]
fn TeamSwitcher(teams: &'static [Team]) -> Element {
    let mut active_team = use_signal(|| 0usize);

    rsx! {
        SidebarMenu {
            SidebarMenuItem {
                DropdownMenu {
                    DropdownMenuTrigger {
                        r#as: move |attributes: Vec<Attribute>| rsx! {
                            SidebarMenuButton {
                                class: DemoStyles::dx_sidebar_menu_disclosure_button,
                                size: SidebarMenuButtonSize::Lg,
                                attributes,
                                div { style: "display:flex; flex-shrink:0; align-items:center; justify-content:center; width:2rem; height:2rem; aspect-ratio:1; border-radius:0.5rem; background:var(--accent); color:var(--accent-fg);",
                                    DemoIcon {}
                                }
                                div { class: DemoStyles::dx_sidebar_info_block,
                                    span { class: DemoStyles::dx_sidebar_info_title, {teams[active_team()].name} }
                                    span { class: DemoStyles::dx_sidebar_info_subtitle, {teams[active_team()].plan} }
                                }
                                ChevronIcon {}
                            }
                        },
                    }
                    Menu {
                        div { style: "padding:0.5rem; font-size:0.75rem; opacity:0.7;",
                            "Teams"
                        }
                        for (idx , team) in teams.iter().enumerate() {
                            MenuItem {
                                index: idx,
                                value: idx,
                                on_select: move |v: usize| active_team.set(v),
                                DemoIcon {}
                                {team.name}
                                span { style: "margin-left:auto; font-size:0.75rem; opacity:0.7;",
                                    "⌘{idx + 1}"
                                }
                            }
                        }
                        MenuSeparator {}
                        MenuItem {
                            index: teams.len(),
                            value: 999usize,
                            on_select: move |_: usize| {},
                            DemoIcon {}
                            div { style: "opacity:0.7; font-weight:500;", "Add team" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavMain(items: &'static [NavMainItem]) -> Element {
    rsx! {
        SidebarGroup {
            SidebarGroupLabel { "Platform" }
            SidebarMenu {
                for item in items.iter() {
                    Collapsible {
                        default_open: item.is_active,
                        r#as: move |attributes: Vec<Attribute>| rsx! {
                            SidebarMenuItem { key: "{item.title}", attributes,
                                CollapsibleTrigger {
                                    r#as: move |attributes: Vec<Attribute>| rsx! {
                                        SidebarMenuButton {
                                            class: DemoStyles::dx_sidebar_menu_disclosure_button,
                                            tooltip: rsx! {
                                                {item.title}
                                            },
                                            attributes,
                                            DemoIcon {}
                                            span { {item.title} }
                                            ChevronIcon {}
                                        }
                                    },
                                }
                                CollapsibleContent {
                                    SidebarMenuSub {
                                        for sub_item in item.items {
                                            SidebarMenuSubItem { key: "{sub_item.title}",
                                                SidebarMenuSubButton {
                                                    r#as: move |attributes: Vec<Attribute>| rsx! {
                                                        a { href: sub_item.url, ..attributes,
                                                            span { {sub_item.title} }
                                                        }
                                                    },
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn NavProjects(projects: &'static [Project]) -> Element {
    rsx! {
        SidebarGroup { class: DemoStyles::dx_sidebar_hide_on_collapse,
            SidebarGroupLabel { "Projects" }
            SidebarMenu {
                for project in projects.iter() {
                    SidebarMenuItem { key: "{project.name}",
                        SidebarMenuButton {
                            r#as: move |attributes: Vec<Attribute>| rsx! {
                                a { href: project.url, ..attributes,
                                    DemoIcon {}
                                    span { {project.name} }
                                }
                            },
                        }
                        DropdownMenu {
                            DropdownMenuTrigger {
                                r#as: move |attributes: Vec<Attribute>| rsx! {
                                    SidebarMenuAction { show_on_hover: true, attributes,
                                        DemoIcon {}
                                        span { style: "position:absolute;overflow:hidden;width:1px;height:1px;padding:0;border:0;margin:-1px;clip-path:inset(50%);white-space:nowrap;",
                                            "More"
                                        }
                                    }
                                },
                            }
                            Menu {
                                MenuItem {
                                    index: 0usize,
                                    value: "view".to_string(),
                                    on_select: move |_: String| {},
                                    DemoIcon {}
                                    span { "View Project" }
                                }
                                MenuItem {
                                    index: 1usize,
                                    value: "share".to_string(),
                                    on_select: move |_: String| {},
                                    DemoIcon {}
                                    span { "Share Project" }
                                }
                                MenuSeparator {}
                                MenuItem {
                                    index: 2usize,
                                    value: "delete".to_string(),
                                    on_select: move |_: String| {},
                                    DemoIcon {}
                                    span { "Delete Project" }
                                }
                            }
                        }
                    }
                }
                SidebarMenuItem {
                    SidebarMenuButton { style: "opacity:0.7; font-weight:500;",
                        DemoIcon {}
                        span { "More" }
                    }
                    SidebarMenuBadge { "+99" }
                }
            }
        }
    }
}

#[component]
fn NavUser() -> Element {
    rsx! {
        SidebarMenu {
            SidebarMenuItem {
                DropdownMenu {
                    DropdownMenuTrigger {
                        r#as: move |attributes: Vec<Attribute>| rsx! {
                            SidebarMenuButton {
                                class: DemoStyles::dx_sidebar_menu_disclosure_button,
                                size: SidebarMenuButtonSize::Lg,
                                attributes,
                                ImageAvatar {
                                    size: AvatarImageSize::Small,
                                    style: "border-radius:0.5rem;",
                                    src: asset!("/assets/dioxus-logo.png").to_string(),
                                    alt: "dioxus avatar",
                                    "DX"
                                }
                                div { class: DemoStyles::dx_sidebar_info_block,
                                    span { class: DemoStyles::dx_sidebar_info_title, "Dioxus" }
                                    span { class: DemoStyles::dx_sidebar_info_subtitle, "m@example.com" }
                                }
                                ChevronIcon {}
                            }
                        },
                    }
                    Menu {
                        div { style: "display:flex; align-items:center; gap:0.5rem; padding:0.375rem 0.25rem; text-align:left; font-size:0.875rem;",
                            ImageAvatar {
                                size: AvatarImageSize::Small,
                                style: "border-radius:0.5rem;",
                                src: asset!("/assets/dioxus-logo.png").to_string(),
                                alt: "dioxus avatar",
                                "DX"
                            }
                            div { class: DemoStyles::dx_sidebar_info_block,
                                span { class: DemoStyles::dx_sidebar_info_title, "Dioxus" }
                                span { class: DemoStyles::dx_sidebar_info_subtitle, "m@example.com" }
                            }
                        }
                        MenuSeparator {}
                        MenuItem {
                            index: 0usize,
                            value: "upgrade".to_string(),
                            on_select: move |_: String| {},
                            DemoIcon {}
                            "Upgrade to Pro"
                        }
                        MenuSeparator {}
                        MenuItem {
                            index: 1usize,
                            value: "account".to_string(),
                            on_select: move |_: String| {},
                            DemoIcon {}
                            "Account"
                        }
                        MenuItem {
                            index: 2usize,
                            value: "billing".to_string(),
                            on_select: move |_: String| {},
                            DemoIcon {}
                            "Billing"
                        }
                        MenuItem {
                            index: 3usize,
                            value: "notifications".to_string(),
                            on_select: move |_: String| {},
                            DemoIcon {}
                            "Notifications"
                        }
                        MenuSeparator {}
                        MenuItem {
                            index: 4usize,
                            value: "logout".to_string(),
                            on_select: move |_: String| {},
                            DemoIcon {}
                            "Log out"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DemoSettingControls(
    side: Signal<SidebarSide>,
    collapsible: Signal<SidebarCollapsible>,
) -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.75rem; padding: 0.75rem; border: 1px solid var(--surface-border); border-radius: 0.75rem; background: var(--surface-hover);",
            div { style: "display: flex; align-items: center; justify-content: space-between; gap: 0.75rem; flex-wrap: wrap;",
                span { style: "font-size: 0.75rem; font-weight: 600; color: var(--fg-muted);",
                    "Side"
                }
                div { style: "display: inline-flex; gap: 0.5rem;",
                    Button {
                        variant: if side() == SidebarSide::Left { ButtonVariant::Default } else { ButtonVariant::Outline },
                        onclick: move |_| side.set(SidebarSide::Left),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "Left"
                    }
                    Button {
                        variant: if side() == SidebarSide::Right { ButtonVariant::Default } else { ButtonVariant::Outline },
                        onclick: move |_| side.set(SidebarSide::Right),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "Right"
                    }
                }
            }
            div { style: "display: flex; align-items: center; justify-content: space-between; gap: 0.75rem; flex-wrap: wrap;",
                span { style: "font-size: 0.75rem; font-weight: 600; color: var(--fg-muted);",
                    "Collapse"
                }
                div { style: "display: inline-flex; gap: 0.5rem; flex-wrap: wrap;",
                    Button {
                        variant: if collapsible() == SidebarCollapsible::Offcanvas { ButtonVariant::Default } else { ButtonVariant::Outline },
                        onclick: move |_| collapsible.set(SidebarCollapsible::Offcanvas),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "Offcanvas"
                    }
                    Button {
                        variant: if collapsible() == SidebarCollapsible::Icon { ButtonVariant::Default } else { ButtonVariant::Outline },
                        onclick: move |_| collapsible.set(SidebarCollapsible::Icon),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "Icon"
                    }
                    Button {
                        variant: if collapsible() == SidebarCollapsible::None { ButtonVariant::Default } else { ButtonVariant::Outline },
                        onclick: move |_| collapsible.set(SidebarCollapsible::None),
                        style: "padding: 0.4rem 0.6rem; font-size: 0.75rem;",
                        "None"
                    }
                }
            }
        }
    }
}

#[component]
fn DemoIcon() -> Element {
    rsx! {
        Circle { class: DemoStyles::dx_sidebar_icon, size: "24px" }
    }
}

#[component]
fn ChevronIcon() -> Element {
    rsx! {
        ChevronRight {
            class: format!("{} {}", DemoStyles::dx_sidebar_icon, DemoStyles::dx_sidebar_chevron),
            size: "24px",
        }
    }
}
