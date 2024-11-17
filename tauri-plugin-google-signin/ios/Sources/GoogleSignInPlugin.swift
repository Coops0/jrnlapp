import SwiftRs
import Tauri
import UIKit
import WebKit
import GoogleSignIn

class GoogleSignInPlugin: Plugin {
    @objc public func request_signin(_ invoke: Invoke) {
        let keyWindow = UIApplication.shared.windows.filter { $0.isKeyWindow }.first;
        
        guard var topController = keyWindow?.rootViewController else {
            invoke.reject("no key window");
            return;
        }
        
        while let presentedViewController = topController.presentedViewController {
            topController = presentedViewController
        }
        
        
        GIDSignIn.sharedInstance.signIn(withPresenting: topController, hint: nil, additionalScopes: ["https://www.googleapis.com/auth/userinfo.profile"]) { result, error in
            guard error == nil else {
                invoke.reject(error?.localizedDescription ?? "unknown error")
                return
            }
            
            guard let result else {
                invoke.reject("result on callback was nill")
                return
            }
            
            print(result)
            invoke.resolve(result.user.idToken?.tokenString)
        }
    }
    
    
    @objc public func signout(_ invoke: Invoke) throws {
        GIDSignIn.sharedInstance.signOut()
        invoke.resolve()
    }
}

@_cdecl("init_plugin_google_signin")
func initPlugin() -> Plugin {
    return GoogleSignInPlugin()
}
