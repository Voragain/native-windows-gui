use nwd::NwgPartial;
use nwg::stretch::{style::{*, Dimension::*}, geometry::*};

use crate::{Project, parser::GuiStruct};


#[derive(Default)]
#[derive(NwgPartial)]
pub struct ObjectInspector {

    #[nwg_layout(
        flex_direction: FlexDirection::Column,
        padding:  Rect { end: Points(10.0), start: Points(10.0), top: Points(5.0), bottom: Points(5.0)  }
    )]
    layout: nwg::FlexboxLayout,

    #[nwg_control]
    pub on_current_gui_changed: nwg::CustomEvent,

    #[nwg_control]
    tt: nwg::Tooltip,

    //
    // Gui structs
    //

    #[nwg_control(flags: "VISIBLE", background_color: Some([255,255,255]))]
    #[nwg_layout_item(layout: layout, flex_shrink: 0.0, size: Size { width: Percent(1.0), height: Points(30.0) })]
    frame1: nwg::Frame,

    #[nwg_layout(parent: frame1)]
    frame1_layout: nwg::FlexboxLayout,

    #[nwg_control(parent: frame1)]
    #[nwg_layout_item(layout: frame1_layout, size: Size { width: Percent(1.0), height: Points(30.0) })]
    #[nwg_events(OnComboxBoxSelection: [ObjectInspector::current_gui_changed])]
    gui_struct_cb: nwg::ComboBox<String>,

    #[nwg_control(parent: frame1, text: "Reload")]
    #[nwg_layout_item(
        layout: frame1_layout,
        flex_shrink: 0.0,
        size: Size { width: Points(100.0), height: Points(30.0) },
        margin: Rect { start: Points(10.0), ..Default::default() }
    )]
    reload_btn: nwg::Button,

    //
    // Current controls list
    //
    #[nwg_control(text: "Ui Component List", background_color: Some([255,255,255]), v_align: nwg::VTextAlign::Top)]
    #[nwg_layout_item(
        layout: layout, flex_shrink: 0.0,
        size: Size { width: Percent(1.0), height: Points(25.0) },
        margin: Rect { top: Points(10.0), ..Default::default() }
    )]
    controls_label: nwg::Label,

    #[nwg_control(
        list_style: nwg::ListViewStyle::Detailed,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::AUTO_COLUMN_SIZE | nwg::ListViewExFlags::FULL_ROW_SELECT,
    )]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Percent(1.0) })]
    control_list: nwg::ListView,

    //
    // Selected control properties
    //
    #[nwg_control(text: "Active Control Properties", background_color: Some([255,255,255]), v_align: nwg::VTextAlign::Top)]
    #[nwg_layout_item(layout: layout, flex_shrink: 0.0,
        size: Size { width: Percent(1.0), height: Points(25.0) },
        margin: Rect { top: Points(10.0), ..Default::default() }
    )]
    properties_label: nwg::Label,

    #[nwg_control(
        list_style: nwg::ListViewStyle::Detailed,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::AUTO_COLUMN_SIZE | nwg::ListViewExFlags::FULL_ROW_SELECT,
    )]
    #[nwg_layout_item(layout: layout, size: Size { width: Percent(1.0), height: Percent(1.0) })]
    properties_list: nwg::ListView,
}

impl ObjectInspector {

    pub(super) fn init(&self) {
        let ctrl = &self.control_list;
        ctrl.set_headers_enabled(true);
        ctrl.insert_column("Name");
        ctrl.insert_column(nwg::InsertListViewColumn {
            text: Some("Type".to_string()),
            width: Some(150),
            .. Default::default()
        });
    
        let prop = &self.properties_list;
        prop.set_headers_enabled(true);
        prop.insert_column("Name");
        prop.insert_column("Value");


        let tt = &self.tt;
        tt.register(&self.gui_struct_cb, "Current GUI struct loaded in the editor");
        tt.register(&self.reload_btn, "Look for new gui struct in the project structure and reload the one that changed");
    
        tt.register(&self.control_list, "GUI control/layout defined in the current GUI struct");
        tt.register(&self.properties_list, "Properties of the selected control");
    }

    /// Returns the selected gui struct index.
    /// The index maps directly to the index of the gui struct in the current project
    /// May return None if no gui struct is selected
    pub fn selected_gui_struct(&self) -> Option<usize> {
        self.gui_struct_cb.selection()
    }

    /// Clear all the controls in the UI
    pub fn clear(&self) {
        self.gui_struct_cb.clear();
        self.control_list.clear();
        self.properties_list.clear();
    }

    /// Reload the project data in the interface
    pub fn reload(&self, project: &Project) {
        self.clear();

        for gui_struct in project.gui_structs() {
            self.gui_struct_cb.push(gui_struct.full_name());
        }

        self.gui_struct_cb.set_selection(Some(0));
        self.on_current_gui_changed.trigger();
    }

    /// Load the current ui struct info in the list
    pub fn select_ui_struct(&self, gui_struct: &GuiStruct) {
        self.control_list.clear();
        self.properties_list.clear();

        for (index, member) in gui_struct.members().iter().enumerate() {
            let item_name = nwg::InsertListViewItem {
                index: Some(index as _),
                column_index: 0,
                text: Some(member.name().to_owned()),
                image: None,
            };

            let ty_name = nwg::InsertListViewItem {
                index: Some(index as _),
                column_index: 1,
                text: Some(member.ty().to_owned()),
                image: None,
            };

            self.control_list.insert_item(item_name);
            self.control_list.insert_item(ty_name);
        }
    }

    /// Enable/Disable the controls in this UI
    pub fn enable_ui(&self, enable: bool) {
        self.gui_struct_cb.set_enabled(enable);
        self.reload_btn.set_enabled(enable);

        // For the change to take effect, the listview needs to be enabled before changing the background color
        match enable {
            false => {
                self.control_list.set_background_color(220, 220, 220);
                self.properties_list.set_background_color(220, 220, 220);
                self.control_list.set_enabled(enable);
                self.properties_list.set_enabled(enable);
            },
            true => {
                self.control_list.set_enabled(enable);
                self.properties_list.set_enabled(enable);
                self.control_list.set_background_color(255, 255, 255);
                self.properties_list.set_background_color(255, 255, 255);
            }
        }
    }

    /// Tells the main GUI that the user changed the current gui struct
    fn current_gui_changed(&self) {
        self.on_current_gui_changed.trigger();
    }

}