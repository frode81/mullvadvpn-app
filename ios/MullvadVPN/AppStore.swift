//
//  AppStore.swift
//  MullvadVPN
//
//  Created by pronebird on 10/03/2020.
//  Copyright © 2020 Mullvad VPN AB. All rights reserved.
//

import Foundation
import StoreKit
import os

class StoreTransactionQueue: NSObject, SKPaymentTransactionObserver {

    private let queue: SKPaymentQueue

    init(queue: SKPaymentQueue) {
        self.queue = queue

        super.init()

        queue.add(self)
    }

    func paymentQueue(_ queue: SKPaymentQueue, updatedTransactions transactions: [SKPaymentTransaction]) {
        for transaction in transactions {
            switch transaction.transactionState {
            case .deferred:
                os_log(.debug, "Deferred %{public}s", transaction.payment.productIdentifier)

            case .failed:
                os_log(.debug, "Failed to purchase %{public}s: %{public}s", transaction.payment.productIdentifier,
                       transaction.error?.localizedDescription ?? "No error")

            case .purchased:
                os_log(.debug, "Purchased %{public}s", transaction.payment.productIdentifier)

            case .purchasing:
                os_log(.debug, "Purchasing %{public}s", transaction.payment.productIdentifier)

            case .restored:
                os_log(.debug, "Restored %{public}s", transaction.payment.productIdentifier)

            @unknown default:
                os_log(.debug, "Unknown transactionState = %{public}d", transaction.transactionState.rawValue)
            }
        }
    }

}
