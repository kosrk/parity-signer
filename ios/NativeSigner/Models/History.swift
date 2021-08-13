//
//  History.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

import Foundation

enum Event: Decodable, Hashable, Equatable {
    case databaseInitiated
    case deviceWasOnline
    case error(String)
    case generalVerifierAdded(Verifier)
    case generalVerifierRemoved(Verifier)
    case historyCleared
    case identitiesWiped
    case identityAdded(IdentityEvent)
    case identityRemoved(IdentityEvent)
    case metadataAdded(MetaSpecs)
    case metadataRemoved(MetaSpecs)
    case metadataVerifierAdded(NetworkVerifierEvent)
    case metadataVerifierRemoved(NetworkVerifierEvent)
    case networkAdded(NewNetwork)
    case networkRemoved(NetworkRemovedEvent)
    case seedsWereAccessed
    case seedsWereShown
    case signedAddNetwork(NewNetwork)
    case signedLoadMetadata(VerifiedMetadataEvent)
    case signedTypes(TypesEvent)
    case systemEntry(String)
    case transactionSigned(SigningEvent)
    case typesInfoUpdated(TypesEvent)
    case userEntry(String)
    case warning(String)
    
    enum CodingKeys: String, CodingKey {
        case event
        case payload
    }
    
    init(from decoder: Decoder) throws {
        let values = try decoder.container(keyedBy: CodingKeys.self)
        let type = try values.decode(String.self, forKey: .event)
        
        switch type {
        case "database_initiated":
            self = .databaseInitiated
        case "device_online":
            self = .deviceWasOnline
        case "error":
            self = .error(try values.decode(String.self, forKey: .payload))
        case "identity_added":
            self = .identityAdded(try values.decode(IdentityEvent.self, forKey: .payload))
        case "seeds_accessed":
            self = .seedsWereAccessed
        case "seeds_shown":
            self = .seedsWereShown
        default:
            self = .error(try values.decode(String.self, forKey: .payload))
        }
    }
}

struct IdentityEvent: Decodable, Hashable {
    var seed_name: String
    var public_key: String
    var path: String
    var network_key: String
}

struct NetworkRemovedEvent: Decodable, Hashable {
    var base58prefix: String
    var color: String
    var decimals: String
    var genesis_hash: String
    var logo: String
    var name: String
    var order: String
    var path_id: String
    var secondary_color: String
    var title: String
    var unit: String
    var verifier: Verifier
}

struct NetworkVerifierEvent: Decodable, Hashable {
    var specname: String
    var verifier: Verifier
}

struct SigningEvent: Decodable, Hashable {
    var transaction: String
    var signed_by: Verifier
}

struct TypesEvent: Decodable, Hashable {
    var types_hash: String
    var verifier: Verifier
}

struct VerifiedMetadataEvent: Decodable, Hashable {
    var specname: String
    var spec_version: String
    var meta_hash: String
    var verifier: Verifier
}

struct History: Decodable {
    var order: Int
    var timestamp: String
    var events: [Event]
}

extension SignerDataModel {
    func getHistory() {
        var err = ExternError()
        let err_ptr: UnsafeMutablePointer<ExternError> = UnsafeMutablePointer(&err)
        let res = print_history(err_ptr, self.dbName)
        if err_ptr.pointee.code == 0 {
            if let historyJSON = String(cString: res!).data(using: .utf8) {
                guard let history = try? JSONDecoder().decode([History].self, from: historyJSON) else {
                    print("JSON decoder failed on history")
                    print(String(cString: res!))
                    print(historyJSON)
                    signer_destroy_string(res!)
                    return
                }
                self.history = history.sorted(by: {$0.order > $1.order})
            } else {
                print("keysJSON corrupted")
            }
            signer_destroy_string(res!)
        } else {
            self.lastError = String(cString: err_ptr.pointee.message)
            print(self.lastError)
            signer_destroy_string(err_ptr.pointee.message)
        }
    }
}
