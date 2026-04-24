package WebShopBehavior {
    state def Created;
    state def PaymentPending;
    state def Paid;
    state def Fulfilled;
    state def Completed;
    state def Failed;

    item def PaymentRequested;
    item def PaymentAuthorized;
    item def InventoryReserved;
    item def ShipmentConfirmed;
    item def PaymentDeclined;
    item def InventoryRejected;

    state def OrderLifecycleStateMachine {
        entry;
        then created;
        state created : Created;
        state paymentPending : PaymentPending;
        state paid : Paid;
        state fulfilled : Fulfilled;
        state completed : Completed;
        state failed : Failed;

        transition to_payment_pending first created accept PaymentRequested then paymentPending;
        transition to_paid first paymentPending accept PaymentAuthorized then paid;
        transition to_fulfilled first paid accept InventoryReserved then fulfilled;
        transition to_completed first fulfilled accept ShipmentConfirmed then completed;
        transition fail_payment first paymentPending accept PaymentDeclined then failed;
        transition fail_inventory first paid accept InventoryRejected then failed;
    }

    action def CheckoutPipeline {
        in cartId : String;
        out checkoutStatus : String;

        action validateCart { out cartValid : Boolean; }
        action authorizePayment { out paymentOk : Boolean; }
        action reserveInventory { out stockReserved : Boolean; }
        action publishOrderCreated { out orderEventPublished : Boolean; }
        action completeCheckout { out done : Boolean; }

        first validateCart then authorizePayment;
        first authorizePayment then reserveInventory;
        first reserveInventory then publishOrderCreated;
        first publishOrderCreated then completeCheckout;
    }
}
