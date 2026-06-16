use dioxus::prelude::*;
use dioxus_components::ButtonVariant;
use dioxus_components::button::Button;
use dioxus_components::dropdown_menu::*;
use dioxus_components::menu::*;

#[component]
pub fn Demo() -> Element {
    let mut selected_destination = use_signal(|| "None".to_string());

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger {
                Button {
                    variant: ButtonVariant::Outline,
                    "Move item"
                }
            }
            Menu {
                MenuLabel { "Choose destination" }
                MenuSub {
                    MenuSubTrigger::<String> {
                        value: "workspace_alpha",
                        index: 0usize,
                        "Workspace Alpha"
                    }
                    MenuSubContent {
                        MenuLabel { "Alpha folders" }
                        MenuSub {
                            MenuSubTrigger::<String> {
                                value: "alpha_projects",
                                index: 0usize,
                                "Workspace Alpha / Projects"
                            }
                            MenuSubContent {
                                MenuLabel { "Project streams" }
                                MenuSub {
                                    MenuSubTrigger::<String> {
                                        value: "alpha_projects_q3",
                                        index: 0usize,
                                        "Workspace Alpha / Projects / Q3"
                                    }
                                    MenuSubContent {
                                        MenuItem::<String> {
                                            value: "alpha_q3_launch",
                                            index: 0usize,
                                            on_select: move |_| {
                                                selected_destination
                                                    .set("Workspace Alpha / Projects / Q3 / Launch".to_string())
                                            },
                                            "Workspace Alpha / Projects / Q3 / Launch"
                                        }
                                        MenuItem::<String> {
                                            value: "alpha_q3_backlog",
                                            index: 1usize,
                                            on_select: move |_| {
                                                selected_destination
                                                    .set("Workspace Alpha / Projects / Q3 / Backlog".to_string())
                                            },
                                            "Workspace Alpha / Projects / Q3 / Backlog"
                                        }
                                    }
                                }
                                MenuItem::<String> {
                                    value: "alpha_projects_archive",
                                    index: 1usize,
                                    on_select: move |_| {
                                        selected_destination
                                            .set("Workspace Alpha / Projects / Archive".to_string())
                                    },
                                    "Workspace Alpha / Projects / Archive"
                                }
                            }
                        }
                        MenuSub {
                            MenuSubTrigger::<String> {
                                value: "alpha_operations",
                                index: 1usize,
                                "Workspace Alpha / Operations"
                            }
                            MenuSubContent {
                                MenuItem::<String> {
                                    value: "alpha_ops_incidents",
                                    index: 0usize,
                                    on_select: move |_| {
                                        selected_destination
                                            .set("Workspace Alpha / Operations / Incidents".to_string())
                                    },
                                    "Workspace Alpha / Operations / Incidents"
                                }
                                MenuItem::<String> {
                                    value: "alpha_ops_runbooks",
                                    index: 1usize,
                                    on_select: move |_| {
                                        selected_destination
                                            .set("Workspace Alpha / Operations / Runbooks".to_string())
                                    },
                                    "Workspace Alpha / Operations / Runbooks"
                                }
                            }
                        }
                    }
                }
                MenuSub {
                    MenuSubTrigger::<String> {
                        value: "workspace_beta",
                        index: 1usize,
                        "Workspace Beta"
                    }
                    MenuSubContent {
                        MenuLabel { "Beta folders" }
                        MenuSub {
                            MenuSubTrigger::<String> {
                                value: "beta_clients",
                                index: 0usize,
                                "Workspace Beta / Clients"
                            }
                            MenuSubContent {
                                MenuItem::<String> {
                                    value: "beta_clients_northwind",
                                    index: 0usize,
                                    on_select: move |_| {
                                        selected_destination
                                            .set("Workspace Beta / Clients / Northwind".to_string())
                                    },
                                    "Workspace Beta / Clients / Northwind"
                                }
                                MenuItem::<String> {
                                    value: "beta_clients_redwood",
                                    index: 1usize,
                                    on_select: move |_| {
                                        selected_destination
                                            .set("Workspace Beta / Clients / Redwood".to_string())
                                    },
                                    "Workspace Beta / Clients / Redwood"
                                }
                            }
                        }
                        MenuSub {
                            MenuSubTrigger::<String> {
                                value: "beta_templates",
                                index: 1usize,
                                "Workspace Beta / Templates"
                            }
                            MenuSubContent {
                                MenuSub {
                                    MenuSubTrigger::<String> {
                                        value: "beta_templates_marketing",
                                        index: 0usize,
                                        "Workspace Beta / Templates / Marketing"
                                    }
                                    MenuSubContent {
                                        MenuItem::<String> {
                                            value: "beta_templates_marketing_email",
                                            index: 0usize,
                                            on_select: move |_| {
                                                selected_destination.set(
                                                    "Workspace Beta / Templates / Marketing / Email".to_string(),
                                                )
                                            },
                                            "Workspace Beta / Templates / Marketing / Email"
                                        }
                                        MenuItem::<String> {
                                            value: "beta_templates_marketing_landing",
                                            index: 1usize,
                                            on_select: move |_| {
                                                selected_destination.set(
                                                    "Workspace Beta / Templates / Marketing / Landing page"
                                                        .to_string(),
                                                )
                                            },
                                            "Workspace Beta / Templates / Marketing / Landing page"
                                        }
                                    }
                                }
                                MenuItem::<String> {
                                    value: "beta_templates_sales",
                                    index: 1usize,
                                    on_select: move |_| {
                                        selected_destination
                                            .set("Workspace Beta / Templates / Sales".to_string())
                                    },
                                    "Workspace Beta / Templates / Sales"
                                }
                            }
                        }
                    }
                }
            }
        }

        p { "Selected destination: {selected_destination}" }
    }
}
