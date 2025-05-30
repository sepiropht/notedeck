use crate::{show_pointer, ProfilePreview};
use egui::Sense;
use enostr::Pubkey;
use nostrdb::{Ndb, Transaction};
use notedeck::{name::get_display_name, Images, NoteAction};

pub struct Mention<'a> {
    ndb: &'a Ndb,
    img_cache: &'a mut Images,
    txn: &'a Transaction,
    pk: &'a [u8; 32],
    selectable: bool,
    size: f32,
}

impl<'a> Mention<'a> {
    pub fn new(
        ndb: &'a Ndb,
        img_cache: &'a mut Images,
        txn: &'a Transaction,
        pk: &'a [u8; 32],
    ) -> Self {
        let size = 16.0;
        let selectable = true;
        Mention {
            ndb,
            img_cache,
            txn,
            pk,
            selectable,
            size,
        }
    }

    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::InnerResponse<Option<NoteAction>> {
        mention_ui(
            self.ndb,
            self.img_cache,
            self.txn,
            self.pk,
            ui,
            self.size,
            self.selectable,
        )
    }
}

impl egui::Widget for Mention<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        self.show(ui).response
    }
}

#[allow(clippy::too_many_arguments)]
#[profiling::function]
fn mention_ui(
    ndb: &Ndb,
    img_cache: &mut Images,
    txn: &Transaction,
    pk: &[u8; 32],
    ui: &mut egui::Ui,
    size: f32,
    selectable: bool,
) -> egui::InnerResponse<Option<NoteAction>> {
    let link_color = ui.visuals().hyperlink_color;

    ui.horizontal(|ui| {
        let profile = ndb.get_profile_by_pubkey(txn, pk).ok();

        let name: String = format!("@{}", get_display_name(profile.as_ref()).name());

        let resp = ui.add(
            egui::Label::new(egui::RichText::new(name).color(link_color).size(size))
                .sense(Sense::click())
                .selectable(selectable),
        );

        let note_action = if resp.clicked() {
            show_pointer(ui);
            Some(NoteAction::Profile(Pubkey::new(*pk)))
        } else if resp.hovered() {
            show_pointer(ui);
            None
        } else {
            None
        };

        if let Some(rec) = profile.as_ref() {
            resp.on_hover_ui_at_pointer(|ui| {
                ui.set_max_width(300.0);
                ui.add(ProfilePreview::new(rec, img_cache));
            });
        }

        note_action
    })
}
