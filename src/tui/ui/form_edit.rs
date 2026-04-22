use crate::tui::app::state::App;
use crate::tui::app::form_edit_state::FormEditState;
use crate::tui::theme::TuiTheme;
use ratatui::prelude::*;
use ratatui::style::{Style, Modifier};
use ratatui::widgets::{Paragraph, Block, Borders, Clear, Wrap};
use ratatui::text::{Line, Span};

pub fn draw_form_edit_modal(f: &mut Frame, app: &App) {
    if let Some(form) = &app.form_edit_state {
        let theme = TuiTheme::from_app(app);
        let area = f.size();
        let modal_width = (area.width as f32 * 0.9) as u16;
        let modal_height = (area.height as f32 * 0.9) as u16;
        let x = (area.width.saturating_sub(modal_width)) / 2;
        let y = (area.height.saturating_sub(modal_height)) / 2;
        let modal_area = Rect { x, y, width: modal_width, height: modal_height };
        
        f.render_widget(Clear, modal_area);
        
        let block = Block::default()
            .title(" Task Editor (Form Mode) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .style(Style::default().bg(theme.background).fg(theme.text));
        
        f.render_widget(block, modal_area);
        
        let inner_area = Rect {
            x: modal_area.x + 1,
            y: modal_area.y + 1,
            width: modal_area.width - 2,
            height: modal_area.height - 2,
        };
        
        // Split the inner area into main form and help section
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(15),    // Main form area
                Constraint::Length(6),  // Help section
            ])
            .split(inner_area);
        
        // Render main form
        render_form_fields(f, chunks[0], app, form);
        
        // Render help section
        render_help_section(f, chunks[1], form, &theme);
    }
}

fn render_form_fields(f: &mut Frame, area: Rect, app: &App, form: &FormEditState) {
    let theme = TuiTheme::from_app(app);
    let mut lines = Vec::new();
    
    // Title field
    let title_style = if form.field_index == 0 {
        Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.info)
    };
    let title_prefix = if form.field_index == 0 { "► " } else { "  " };
    lines.push(Line::from(vec![
        Span::styled(title_prefix, Style::default().fg(theme.warning)),
        Span::styled("Title: ", title_style),
        Span::styled(&form.title, if form.field_index == 0 { 
            theme.active_field_style()
        } else { 
            theme.inactive_field_style()
        }),
    ]));
    
    // Description field
    let desc_style = if form.field_index == 1 {
        Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.info)
    };
    let desc_prefix = if form.field_index == 1 { "► " } else { "  " };
    let desc_text = if form.description.is_empty() { 
        "<empty>" 
    } else { 
        &form.description 
    };
    lines.push(Line::from(vec![
        Span::styled(desc_prefix, Style::default().fg(theme.warning)),
        Span::styled("Description: ", desc_style),
        Span::styled(desc_text, if form.field_index == 1 { 
            theme.active_field_style()
        } else { 
            theme.inactive_field_style()
        }),
    ]));
    
    // Due Date field
    let due_style = if form.field_index == 2 {
        Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.info)
    };
    let due_prefix = if form.field_index == 2 { "► " } else { "  " };
    let due_text = form.due_date.as_deref().unwrap_or("<not set>");
    lines.push(Line::from(vec![
        Span::styled(due_prefix, Style::default().fg(theme.warning)),
        Span::styled("Due Date: ", due_style),
        Span::styled(due_text, if form.field_index == 2 { 
            theme.active_field_style()
        } else { 
            theme.inactive_field_style()
        }),
        Span::styled(" (YYYY-MM-DD)", Style::default().fg(theme.subtle_text)),
    ]));
    
    // Start Date field
    let start_style = if form.field_index == 3 {
        Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.info)
    };
    let start_prefix = if form.field_index == 3 { "► " } else { "  " };
    let start_text = form.start_date.as_deref().unwrap_or("<not set>");
    lines.push(Line::from(vec![
        Span::styled(start_prefix, Style::default().fg(theme.warning)),
        Span::styled("Start Date: ", start_style),
        Span::styled(start_text, if form.field_index == 3 { 
            theme.active_field_style()
        } else { 
            theme.inactive_field_style()
        }),
        Span::styled(" (YYYY-MM-DD)", Style::default().fg(theme.subtle_text)),
    ]));
    
    // Priority field
    let prio_style = if form.field_index == 4 {
        Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.info)
    };
    let prio_prefix = if form.field_index == 4 { "► " } else { "  " };
    let prio_text = form.priority.map(|p| p.to_string()).unwrap_or_else(|| "0".to_string());
    lines.push(Line::from(vec![
        Span::styled(prio_prefix, Style::default().fg(theme.warning)),
        Span::styled("Priority: ", prio_style),
        Span::styled(&prio_text, if form.field_index == 4 { 
            theme.active_field_style()
        } else { 
            theme.inactive_field_style()
        }),
        Span::styled(" (0-5, 0=none)", Style::default().fg(theme.subtle_text)),
    ]));
    
    // Project field (with name lookup)
    let proj_style = if form.field_index == 5 {
        Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.info)
    };
    let proj_prefix = if form.field_index == 5 { "► " } else { "  " };
    let project_name = app.project_map.get(&form.project_id)
        .map(|name| format!("{} (ID: {})", name, form.project_id))
        .unwrap_or_else(|| format!("Unknown Project (ID: {})", form.project_id));
    lines.push(Line::from(vec![
        Span::styled(proj_prefix, Style::default().fg(theme.warning)),
        Span::styled("Project: ", proj_style),
        Span::styled(&project_name, if form.field_index == 5 { 
            theme.active_field_style()
        } else { 
            theme.inactive_field_style()
        }),
        Span::styled(" (Space to pick)", Style::default().fg(theme.subtle_text)),
    ]));
    
    // Labels field (with name lookup)
    let labels_style = if form.field_index == 6 {
        Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.info)
    };
    let labels_prefix = if form.field_index == 6 { "► " } else { "  " };
    let labels_text = if form.label_ids.is_empty() {
        "<none>".to_string()
    } else {
        form.label_ids.iter()
            .filter_map(|id| app.label_map.get(id))
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    };
    lines.push(Line::from(vec![
        Span::styled(labels_prefix, Style::default().fg(theme.warning)),
        Span::styled("Labels: ", labels_style),
        Span::styled(&labels_text, if form.field_index == 6 { 
            theme.active_field_style()
        } else { 
            theme.inactive_field_style()
        }),
        Span::styled(" (Space to pick)", Style::default().fg(theme.subtle_text)),
    ]));
    
    // Assignees field
    let assign_style = if form.field_index == 7 {
        Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.info)
    };
    let assign_prefix = if form.field_index == 7 { "► " } else { "  " };
    let assign_text = if form.assignee_ids.is_empty() { 
        "<none>" 
    } else { 
        &format!("{:?}", form.assignee_ids) 
    };
    lines.push(Line::from(vec![
        Span::styled(assign_prefix, Style::default().fg(theme.warning)),
        Span::styled("Assignees: ", assign_style),
        Span::styled(assign_text, if form.field_index == 7 { 
            theme.active_field_style()
        } else { 
            theme.inactive_field_style()
        }),
    ]));
    
    // Is Favorite field
    let fav_style = if form.field_index == 8 {
        Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.info)
    };
    let fav_prefix = if form.field_index == 8 { "► " } else { "  " };
    let fav_text = if form.is_favorite { "★ Yes" } else { "☆ No" };
    lines.push(Line::from(vec![
        Span::styled(fav_prefix, Style::default().fg(theme.warning)),
        Span::styled("Favorite: ", fav_style),
        Span::styled(fav_text, if form.field_index == 8 { 
            theme.active_field_style()
        } else { 
            theme.inactive_field_style()
        }),
        Span::styled(" (Space to toggle)", Style::default().fg(theme.subtle_text)),
    ]));
    
    // Comment field
    let comment_style = if form.field_index == 9 {
        Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.info)
    };
    let comment_prefix = if form.field_index == 9 { "► " } else { "  " };
    let comment_text = if form.comment.is_empty() { 
        "Type your comment, then press Enter to save" 
    } else { 
        &form.comment 
    };
    lines.push(Line::from(vec![
        Span::styled(comment_prefix, Style::default().fg(theme.warning)),
        Span::styled("Add Comment: ", comment_style),
        Span::styled(comment_text, if form.field_index == 9 { 
            theme.active_field_style()
        } else { 
            theme.inactive_field_style()
        }),
    ]));
    
    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(theme.text).bg(theme.background));
    
    f.render_widget(paragraph, area);
    // Position the terminal cursor at the current field's edit position
    // Only for text/editable fields: title (0), description (1), due_date (2), start_date (3), priority (4), comment (9)
    if (0..=4).contains(&form.field_index) || form.field_index == 9 {
        // Determine label length for cursor offset
        let label = match form.field_index {
            0 => "Title: ",
            1 => "Description: ",
            2 => "Due Date: ",
            3 => "Start Date: ",
            4 => "Priority: ",
            9 => "Add Comment: ",
            _ => "",
        };
        let prefix_width = 2; // "► " or "  "
        let offset = prefix_width + label.len();
        let cursor_x = area.x + offset as u16 + form.cursor_position as u16;
        let cursor_y = area.y + form.field_index as u16;
        f.set_cursor(cursor_x, cursor_y);
    }
}

fn render_help_section(f: &mut Frame, area: Rect, form: &FormEditState, theme: &TuiTheme) {
    let mut help_lines = Vec::new();
    
    help_lines.push(Line::from(vec![
        Span::styled("Navigation:", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled("Tab", Style::default().fg(theme.warning)),
        Span::raw("/"),
        Span::styled("Shift+Tab", Style::default().fg(theme.warning)),
        Span::raw(" - Next/Previous field  "),
        Span::styled("↑↓", Style::default().fg(theme.warning)),
        Span::raw(" - Navigate fields"),
    ]));
    
    help_lines.push(Line::from(vec![
        Span::styled("Editing:", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::raw("     "),
        Span::styled("Type", Style::default().fg(theme.warning)),
        Span::raw(" - Edit text fields  "),
        Span::styled("Backspace", Style::default().fg(theme.warning)),
        Span::raw(" - Delete  "),
        Span::styled("Space", Style::default().fg(theme.warning)),
        Span::raw(" - Pick/Toggle"),
    ]));
    
    help_lines.push(Line::from(vec![
        Span::styled("Actions:", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::raw("     "),
        Span::styled("Enter", Style::default().fg(theme.success)),
        Span::raw(" - Save task  "),
        Span::styled("Esc", Style::default().fg(theme.danger)),
        Span::raw(" - Cancel without saving"),
    ]));
    
    // Field-specific help
    match form.field_index {
        2 | 3 => {
            help_lines.push(Line::from(vec![
                Span::styled("Date Format:", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                Span::raw("  YYYY-MM-DD (e.g., 2025-07-15) or leave empty for no date"),
            ]));
        }
        4 => {
            help_lines.push(Line::from(vec![
                Span::styled("Priority:", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                Span::raw("     0=None, 1=Low, 2=Medium, 3=High, 4=Urgent, 5=Critical"),
            ]));
        }
        5 => {
            help_lines.push(Line::from(vec![
                Span::styled("Project:", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                Span::raw("      Press "),
                Span::styled("Space", Style::default().fg(theme.warning)),
                Span::raw(" to open project picker"),
            ]));
        }
        6 => {
            help_lines.push(Line::from(vec![
                Span::styled("Labels:", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                Span::raw("       Press "),
                Span::styled("Space", Style::default().fg(theme.warning)),
                Span::raw(" to open label picker"),
            ]));
        }
        8 => {
            help_lines.push(Line::from(vec![
                Span::styled("Favorite:", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                Span::raw("     Press "),
                Span::styled("Space", Style::default().fg(theme.warning)),
                Span::raw(" to toggle favorite status"),
            ]));
        }
        _ => {
            help_lines.push(Line::from(vec![
                Span::styled("Tip:", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                Span::raw("          Use Tab to navigate between fields quickly"),
            ]));
        }
    }
    
    let help_paragraph = Paragraph::new(help_lines)
        .block(Block::default()
            .title(" Help ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .style(Style::default().bg(theme.surface).fg(theme.text)))
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(theme.text).bg(theme.surface));
    
    f.render_widget(help_paragraph, area);
}
