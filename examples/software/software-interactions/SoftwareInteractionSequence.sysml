package SoftwareInteractionSequence {
    private import SoftwareInteractions::*;

    part def RetryAuthorization :> InteractionScenario {
        part eventBus : Lifeline;
        part paymentsWorker : Lifeline;

        part redeliver : AsynchronousMessage {
            ref from : Lifeline = eventBus;
            ref to : Lifeline = paymentsWorker;
            attribute :>> label = "payment.retry";
        }
    }

    part def CheckoutAuthorization :> InteractionScenario {
        part storefront : Lifeline;
        part checkoutApi : Lifeline;
        part paymentsService : Lifeline;
        part paymentRecord : Lifeline;

        part submitOrder : SynchronousCall {
            ref from : Lifeline = storefront;
            ref to : Lifeline = checkoutApi;
            attribute :>> label = "POST /checkout";
        }

        part authorizePayment : SynchronousCall {
            ref from : Lifeline = checkoutApi;
            ref to : Lifeline = paymentsService;
            attribute :>> label = "authorizePayment()";
        }

        part createPaymentRecord : CreationMessage {
            ref from : Lifeline = paymentsService;
            ref to : Lifeline = paymentRecord;
            attribute :>> label = "create payment record";
        }

        part paymentExecution : Activation {
            ref on : Lifeline = paymentsService;
            ref startMessage : Message = authorizePayment;
            ref finishMessage : Message = checkoutAccepted;
        }

        part authorizationOutcome : AltFragment {
            part approved : InteractionOperand {
                attribute :>> guard = "payment approved";

                part checkoutAccepted : ReturnMessage {
                    ref from : Lifeline = paymentsService;
                    ref to : Lifeline = checkoutApi;
                    attribute :>> label = "approved";
                }

                part sendConfirmation : SynchronousCall {
                    ref from : Lifeline = checkoutApi;
                    ref to : Lifeline = storefront;
                    attribute :>> label = "201 Created";
                }
            }

            part declined : InteractionOperand {
                attribute :>> guard = "payment declined";

                part retries : LoopFragment {
                    part repeatAttempt : InteractionOperand {
                        attribute :>> guard = "retry attempts remaining";

                        part retryAuthorization : InteractionRef {
                            ref target : InteractionScenario = RetryAuthorization;
                        }
                    }
                }

                part checkoutRejected : ReturnMessage {
                    ref from : Lifeline = paymentsService;
                    ref to : Lifeline = checkoutApi;
                    attribute :>> label = "declined";
                }

                part rejectOrder : ReturnMessage {
                    ref from : Lifeline = checkoutApi;
                    ref to : Lifeline = storefront;
                    attribute :>> label = "402 Payment Required";
                }
            }
        }
    }
}
