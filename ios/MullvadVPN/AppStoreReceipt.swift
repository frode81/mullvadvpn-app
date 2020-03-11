//
//  AppStoreReceipt.swift
//  MullvadVPN
//
//  Created by pronebird on 11/03/2020.
//  Copyright © 2020 Mullvad VPN AB. All rights reserved.
//

import Foundation

enum AppStoreReceipt {
    enum Error: Swift.Error {
        /// AppStore receipt file does not exist or file URL is not available
        case doesNotExist

        /// IO error
        case io(Swift.Error)
    }

    static func read() -> Result<Data, Error> {
        guard let appStoreReceiptURL = Bundle.main.appStoreReceiptURL else {
            return .failure(.doesNotExist)
        }

        return Result { try Data(contentsOf: appStoreReceiptURL) }
            .mapError { (error) -> Error in
                if let ioError = error as? CocoaError, ioError.code == .fileNoSuchFile {
                    return .doesNotExist
                } else {
                    return .io(error)
                }
        }
    }
}
