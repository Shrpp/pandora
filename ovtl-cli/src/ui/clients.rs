use ratatui::{layout::Rect, Frame};

use crate::{app::App, components::table::StatefulTable};

pub fn render(frame: &mut Frame, app: &App, area: Rect, table: &mut StatefulTable) {
    let tenant_name = app.active_tenant_name().unwrap_or("?");
    let title = format!(" Clients — {tenant_name} ");

    let rows: Vec<Vec<String>> = app
        .clients
        .iter()
        .map(|c| {
            vec![
                c.id[..8].to_string() + "…",
                c.name.clone(),
                c.client_id[..8].to_string() + "…",
                c.scopes.join(" "),
                if c.is_active { "active".into() } else { "inactive".into() },
            ]
        })
        .collect();

    table.select(app.client_selected);
    table.render(frame, area, &title, &["ID", "Name", "Client ID", "Scopes", "Active"], rows);
}
