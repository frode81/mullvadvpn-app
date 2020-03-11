//
//  SKRequestPublisher.swift
//  MullvadVPN
//
//  Created by pronebird on 11/03/2020.
//  Copyright © 2020 Mullvad VPN AB. All rights reserved.
//

import Combine
import Foundation
import StoreKit

/// A protocol that formalizes the kind of `Subscription` expected by `SKRequestPublisher`
protocol SKRequestSubscriptionProtocol: Subscription {
    associatedtype Output
    associatedtype Failure: Error

    init<S: Subscriber>(request: SKRequest, subscriber: S) where S.Input == Output, S.Failure == Failure
}

/// A basic `SKRequest` subscription implementation that does not emit any values
class SKRequestSubscription<ResponseType>: NSObject, Subscription, SKRequestDelegate, SKRequestSubscriptionProtocol {

    typealias Output = ResponseType
    typealias Failure = Error

    private let request: SKRequest
    private var requestError: Error?

    fileprivate let subscriber: AnySubscriber<Output, Failure>

    required init<S: Subscriber>(request: SKRequest, subscriber: S) where S.Input == Output, S.Failure == Failure {
        self.request = request
        self.subscriber = AnySubscriber(subscriber)
    }

    func request(_ demand: Subscribers.Demand) {
        request.start()
    }

    func cancel() {
        request.cancel()
    }

    // MARK: - SKRequestDelegate

    func request(_ request: SKRequest, didFailWithError error: Error) {
        requestError = error
    }

    func requestDidFinish(_ request: SKRequest) {
        subscriber.receive(completion: requestError.flatMap { .failure($0) } ?? .finished)
    }
}

/// A subclass of `SKRequestSubscription` that emits the `SKProductsResponse` response
class SKProductsRequestSubscription: SKRequestSubscription<SKProductsResponse>, SKProductsRequestDelegate {

    // MARK: - SKProductsRequestDelegate

    func productsRequest(_ request: SKProductsRequest, didReceive response: SKProductsResponse) {
        _ = self.subscriber.receive(response)
    }

}

class SKRequestPublisher<SubscriptionType>: Publisher
    where SubscriptionType: SKRequestSubscriptionProtocol
{
    typealias Output = SubscriptionType.Output
    typealias Failure = SubscriptionType.Failure

    fileprivate let request: SKRequest

    init(request: SKRequest) {
        self.request = request
    }

    func receive<S>(subscriber: S) where S : Subscriber, Failure == S.Failure, Output == S.Input {
        let subscription = SubscriptionType(request: self.request, subscriber: subscriber)
        
        subscriber.receive(subscription: subscription)
    }

}

protocol SKRequestPublishing {
    associatedtype SubscriptionType: SKRequestSubscriptionProtocol

    var publisher: SKRequestPublisher<SubscriptionType> { get }
}

extension SKProductsRequest: SKRequestPublishing {
    var publisher: SKRequestPublisher<SKProductsRequestSubscription> {
        return .init(request: self)
    }
}

extension SKReceiptRefreshRequest: SKRequestPublishing {
    var publisher: SKRequestPublisher<SKRequestSubscription<Never>> {
        return .init(request: self)
    }
}
