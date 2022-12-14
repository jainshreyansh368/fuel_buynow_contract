library errors;

pub enum InputError {
    AdminDoesNotExist: (),
    PriceCantBeZero: (),
    OffererNotExists: (),
    LessPriceThanPreviousOffer: (), 
}

pub enum AccessError {
    NFTAlreadyListed: (),
    NFTNotListed: (),
    SenderCannotSetAccessControl: (),
    SenderNotAdmin: (),
    SenderNotOwner: (),
    SenderNotOwnerOrApproved: (),
    SenderDidNotMakeOffer: (),
    BuyerSameAsSeller: (),
}
