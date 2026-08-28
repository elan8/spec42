package InteractionDemo {
	item def CheckoutRequest;
	port def CheckoutApi;
	// A lifeline's type may carry ports and nested parts; those are not lifelines.
	part def Storefront { port checkoutApi : CheckoutApi; }
	part def ApiGateway { port checkoutApi : ~CheckoutApi; part edgeCache; }
	part def CheckoutService;

	// A sequence scenario in the standard SysML v2 interaction vocabulary:
	// lifelines are typed parts carrying ordered event occurrences, messages
	// connect a send event to a receive event, and succession fixes their order.
	occurrence def CheckoutFlow {
		part storefront : Storefront {
			event occurrence submitSent;
			then event occurrence outcomeReceived;
		}
		part apiGateway : ApiGateway {
			event occurrence submitReceived;
			then event occurrence forwardSent;
			then event occurrence outcomeForwarded;
		}
		part checkoutService : CheckoutService {
			event occurrence forwardReceived;
			then event occurrence outcomeSent;
		}

		message submitCheckout of CheckoutRequest
			from storefront.submitSent to apiGateway.submitReceived;
		message forwardCheckout
			from apiGateway.forwardSent to checkoutService.forwardReceived;
		message checkoutOutcome
			from checkoutService.outcomeSent to apiGateway.outcomeForwarded;
		message apiResponse
			from apiGateway.outcomeForwarded to storefront.outcomeReceived;

		succession first submitCheckout then forwardCheckout;
		succession first forwardCheckout then checkoutOutcome;
		succession first checkoutOutcome then apiResponse;
	}
}
package Views {
	private import StandardViewDefinitions::*;
	import InteractionDemo::*;

	view checkoutFlow : SequenceView {
		expose InteractionDemo::CheckoutFlow;
	}
}
