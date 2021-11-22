//
//  Footer.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 26.7.2021.
//

import SwiftUI

struct KeySymbol: View {
    var body: some View {
        VStack{
            Image(systemName: "circle.hexagongrid.fill").imageScale(.medium)
        }
    }
}

struct WrenchSymbol: View {
    var body: some View {
        VStack{
            Image(systemName: "gearshape.fill").imageScale(.medium)
        }
    }
}

struct Footer: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        HStack {
            Button(action: {
                data.refreshUI()
                data.signerScreen = .history
            }) {
                VStack(alignment: .center) {
                    Image(systemName: "rectangle.grid.1x2.fill").imageScale(.medium).foregroundColor(data.signerScreen == .history ? Color("buttonActive") : Color("buttonPassiveImage"))
                    Text("Log").foregroundColor(data.signerScreen == .history ? Color("buttonActive") : Color("buttonPassiveText"))
                }
            }
            Spacer()
            Button(action: {
                data.refreshUI()
                data.signerScreen = .scan
            }) {
                VStack {
                    Image(systemName: "viewfinder").imageScale(.medium).foregroundColor(data.signerScreen == .scan ? Color("buttonActive") : Color("buttonPassiveImage"))
                    Text("Scanner").foregroundColor(data.signerScreen == .scan ? Color("buttonActive") : Color("buttonPassiveText"))
                }
            }
            Spacer()
            Button(action: {
                data.refreshUI()
                data.signerScreen = .keys
            }) {
                VStack{
                    KeySymbol().foregroundColor(data.signerScreen == .keys ? Color("buttonActive") : Color("buttonPassiveImage"))
                    Text("Keys").foregroundColor(data.signerScreen == .keys ? Color("buttonActive") : Color("buttonPassiveText"))
                }
            }
            Spacer()
            Button(action: {
                data.refreshUI()
                data.networkSettings = nil
                data.signerScreen = .settings
            }) {
                VStack {
                    WrenchSymbol().foregroundColor(data.signerScreen == .settings ? Color("buttonActive") : Color("buttonPassiveImage"))
                    Text("Settings").foregroundColor(data.signerScreen == .settings ? Color("buttonActive") : Color("buttonPassiveText"))
                }
            }
        }.font(.footnote)
    }
}

/*
 struct Footer_Previews: PreviewProvider {
 static var previews: some View {
 Footer().previewLayout(.sizeThatFits)
 }
 }
 */
