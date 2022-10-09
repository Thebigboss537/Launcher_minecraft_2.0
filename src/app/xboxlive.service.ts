import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class XboxliveService {

  constructor(private http: HttpClient) { }

  

  AuthenticatewithXboxLive(token:any){
    let data = `{
      "Properties": {
        "AuthMethod": "RPS",
        "SiteName": "user.auth.xboxlive.com",
        "RpsTicket": "d=${token}"
        },
      "RelyingParty": "http://auth.xboxlive.com",
      "TokenType": "JWT"
      }`;

      const headers = new HttpHeaders({
        'Content-Type': 'application/json',
        'Accept': 'application/json'
      })

    return this.http.post('https://user.auth.xboxlive.com/user/authenticate', data, { headers: headers});
  }

  ObtainXSTS(xbltoken:any){
    let data = `{
      "Properties": {
          "SandboxId": "RETAIL",
          "UserTokens": [
              "${xbltoken}" 
          ]
      },
      "RelyingParty": "rp://api.minecraftservices.com/",
      "TokenType": "JWT"
   }`;

      const headers = new HttpHeaders({
        'Content-Type': 'application/json',
        'Accept': 'application/json'
      })

    return this.http.post('https://xsts.auth.xboxlive.com/xsts/authorize', data, { headers: headers});
  }


  AuthenticatewithMinecraft(userhash : any, xsts_token: any){
    let data = `{
      "identityToken" : "XBL3.0 x=${userhash};
      ${xsts_token}",
      "ensureLegacyEnabled" : true
    }`;

    const headers = new HttpHeaders({
      'Content-Type': 'application/json',
      'Accept': 'application/json',
      'Access-Control-Allow-Origin' : '*'
    })
    return this.http.post('https://api.minecraftservices.com/authentication/login_with_xbox', data, { headers: headers});
}


  /*CheckingGameOwnership(tokenminecraft : any){
    
    const headers = new HttpHeaders({
      'Content-Type': 'application/json',
      'Accept': 'application/json',
      'Authorization': `Bearer ${tokenminecraft}`
    })

    return this.http.get('https://api.minecraftservices.com/entitlements/mcstore', { headers: headers});
  }*/
}
