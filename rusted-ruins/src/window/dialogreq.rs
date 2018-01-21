
use common::gamedata::chara::{CharaId, CharaTalk};
use game::{Game, DialogOpenRequest, TalkStatus};
use super::DialogWindow;
use super::talkwindow;
use super::msgdialog;

pub fn create_dialog_from_request(req: DialogOpenRequest, game: &mut Game) -> Option<Box<DialogWindow>> {
    Some(match req {
        DialogOpenRequest::YesNo { mut callback, msg_text_id } => {
            let msgdialog = msgdialog::MsgDialog::with_yesno(
                ::text::ui_txt(msg_text_id),
                move |mut pa, n| {
                    callback(pa, n == 0);
                    super::DialogResult::Close
                }
            );
            Box::new(msgdialog)
        }
        DialogOpenRequest::Talk { chara_talk, cid } => {
            create_talk_dialog(chara_talk, cid, game)?
        }
    })
}

pub fn create_talk_dialog(
    chara_talk: CharaTalk, cid: CharaId, game: &mut Game) -> Option<Box<DialogWindow>> {
        
    let talk_status = TalkStatus::new(chara_talk, cid, game)?;
    
    let talk_window = talkwindow::TalkWindow::new(talk_status);
    Some(Box::new(talk_window))
}

