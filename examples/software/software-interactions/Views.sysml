package Views {
    import SoftwareInteractionSequence::*;

    view checkoutAuthorization : SequenceView {
        expose SoftwareInteractionSequence::CheckoutAuthorization;
    }

    view retryAuthorization : SequenceView {
        expose SoftwareInteractionSequence::RetryAuthorization;
    }
}
