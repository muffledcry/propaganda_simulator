pub enum PolAlignment {
    FarRight,
    Right,
    CenterRight,
    Center,
    CenterLeft,
    Left,
    RadicalLeft,
}

pub enum OutletFundingType {
    Advertising,
    Donation,
    Subscription,
}

pub enum MessageType {
    Report,
    Opinion,
    Attack,
}

pub struct Citizen {
    pub pol_alignment: PolAlignment,
    pub messages_received: u32,
    pub receptivity: u32,
    pub groupness: u32,
}

pub struct Outlet {
    pub pol_alignment: PolAlignment,
    pub funding: Vec<OutletFundingType>,
    pub readership: u32,
}

pub struct Message {
    pub pol_alignment: PolAlignment,
    pub message_type: MessageType, 
}


fn main() {
    println!("Hello, world!");
}
