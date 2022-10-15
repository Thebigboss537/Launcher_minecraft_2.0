import { Component, OnInit } from '@angular/core';
import { MsalBroadcastService, MsalService } from '@azure/msal-angular';
import { InteractionStatus, AuthenticationResult } from '@azure/msal-browser';
import { filter, Subject, takeUntil } from 'rxjs';
import { XboxliveService } from '../xboxlive.service';
import { invoke } from '@tauri-apps/api/tauri'


@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent implements OnInit {
  
  xbltoken = '';
  xststoken = '';
  tokenminecraft = '';
  uhs = '';
  username = '';
  private token = '';
  loginDisplay = false;
  loginRequest = {
    scopes: ["XboxLive.signin"]
  };
  private readonly _destroying$ = new Subject<void>();
  constructor(private broadcastService: MsalBroadcastService,private authService: MsalService, private xbox: XboxliveService) { }

  ngOnInit(): void {
    /*this.broadcastService.inProgress$
    .pipe(
      filter((status: InteractionStatus) => status === InteractionStatus.None),
      takeUntil(this._destroying$)
    )
    .subscribe(() => {
      this.setLoginDisplay();
    })*/
    
  }
  
  login() {
    this.authService.loginPopup(this.loginRequest).subscribe((obsevable : AuthenticationResult) => {
      this.token = obsevable.accessToken;
      //this.obtaintokenminecraft();
      this.xboxlive();
    });
    
  }

  
  xboxlive(){
    this.xbox.AuthenticatewithXboxLive(this.token).subscribe((a : any)=>{
      console.log(a);
      this.xbltoken = a.Token;
      this.uhs = a.DisplayClaims.xui[0].uhs;
      this.obtainxststoken();
    })
  }

  obtainxststoken(){
    this.xbox.ObtainXSTS(this.xbltoken).subscribe((b : any)=>{
      this.xststoken = b.Token;
      //this.obtaintokenminecraft();
      //invoke('principal');

      invoke("principal", { userhash: this.uhs.toString(), xststoken: this.xststoken.toString() }).then((text: string) => {
        this.username = text;
      });
      /*invoke<string>("principal").then((text : any) => {
        console.log(text);
      });*/
    })
  }

  




  setLoginDisplay() {
    this.loginDisplay = this.authService.instance.getAllAccounts().length > 0;
  }

  ngOnDestroy(): void {
    this._destroying$.next(undefined);
    this._destroying$.complete();
  }
}
