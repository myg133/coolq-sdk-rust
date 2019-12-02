use super::{Event, Events};

pub struct GroupMemberDecreaseEvent {
    pub(crate) canceld: bool,
    pub sub_type: i32,
    pub send_time: i32,
    pub group_id: i64,
    pub operate_user_id: i64,
    pub being_operate_user_id: i64
}

impl Event for GroupMemberDecreaseEvent {
    fn get_type(&self) -> Events { Events::GroupMemberDecrease }

    fn is_cancel(&self) -> bool {
        self.canceld
    }

    fn cancel(&mut self) {
        self.canceld = true;
    }
}