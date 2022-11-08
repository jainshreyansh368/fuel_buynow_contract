library errors;

pub enum InputError {
    AdminDoesNotExist: (),
    PriceCantBeZero: (),
    OffererNotExists: (), 
}

pub enum AccessError {
    NFTAlreadyListed: (),
    NFTNotListed: (),
    SenderCannotSetAccessControl: (),
    SenderNotAdmin: (),
    SenderNotOwner: (),
    SenderNotOwnerOrApproved: (),
    SenderDidNotMakeOffer: (),
}
