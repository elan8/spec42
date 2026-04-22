package CommerceBehavior {
    state def Created;
    state def Priced;
    state def PaymentPending;
    state def Paid;
    state def Fulfilled;
    state def Completed;
    state def Cancelled;
    state def Failed;

    item def PriceCalculated;
    item def PaymentRequested;
    item def PaymentAuthorized;
    item def InventoryReserved;
    item def ShipmentConfirmed;
    item def OrderClosed;
    item def PaymentDeclined;
    item def InventoryRejected;
    item def CancelRequested;

    state def OrderLifecycleStateMachine {
        entry;
        then created;
        state created : Created;
        state priced : Priced;
        state paymentPending : PaymentPending;
        state paid : Paid;
        state fulfilled : Fulfilled;
        state completed : Completed;
        state cancelled : Cancelled;
        state failed : Failed;

        transition to_priced first created accept PriceCalculated then priced;
        transition to_payment_pending first priced accept PaymentRequested then paymentPending;
        transition to_paid first paymentPending accept PaymentAuthorized then paid;
        transition to_fulfilled first paid accept InventoryReserved then fulfilled;
        transition to_completed first fulfilled accept ShipmentConfirmed then completed;
        transition complete_order first completed accept OrderClosed then completed;
        transition cancel_from_created first created accept CancelRequested then cancelled;
        transition cancel_from_priced first priced accept CancelRequested then cancelled;
        transition fail_payment first paymentPending accept PaymentDeclined then failed;
        transition fail_inventory first paid accept InventoryRejected then failed;
    }

    action def CheckoutFlow {
        in cartId : String;
        out checkoutStatus : String;

        action validateCart { out cartValid : Boolean; }
        action enrichPricing { out priced : Boolean; }
        action authorizePayment { out paymentOk : Boolean; }
        action reserveInventory { out stockReserved : Boolean; }
        action publishOrderCreated { out orderEventPublished : Boolean; }
        action sendConfirmation { out confirmationSent : Boolean; }
        action completeCheckout { out done : Boolean; }

        first validateCart then enrichPricing;
        first enrichPricing then authorizePayment;
        first authorizePayment then reserveInventory;
        first reserveInventory then publishOrderCreated;
        first publishOrderCreated then sendConfirmation;
        first sendConfirmation then completeCheckout;
    }
}
