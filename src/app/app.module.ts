import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { HttpClientModule } from '@angular/common/http';

import { AppComponent } from './app.component';
import { AppRouterModule } from './app-router.module';
import { LoginComponent } from './login/login.component';
import { IniciarMinecraftComponent } from './iniciar-minecraft/iniciar-minecraft.component';

import { MsalModule, MsalRedirectComponent, MsalGuard } from '@azure/msal-angular'; // Updated import
import { InteractionType,PublicClientApplication } from '@azure/msal-browser';
import { XboxliveService } from './xboxlive.service';

const isIE = window.navigator.userAgent.indexOf('MSIE ') > -1 || window.navigator.userAgent.indexOf('Trident/') > -1;



// @NgModule decorator with its metadata
@NgModule({
  declarations: [
    AppComponent, 
    LoginComponent, 
    IniciarMinecraftComponent
   ],
   
  imports: [BrowserModule, AppRouterModule, HttpClientModule, 
    MsalModule.forRoot( new PublicClientApplication({
      auth: {
        clientId: '04e788a5-9628-4244-a9f3-8583a9351800',
        authority: 'https://login.microsoftonline.com/consumers',
        redirectUri: 'http://localhost:1420'
      },
      cache: {
        cacheLocation: 'localStorage',
        storeAuthStateInCookie: isIE
      }
    }),null,{
      interactionType: InteractionType.Redirect, // MSAL Interceptor Configuration
      protectedResourceMap: new Map([ 
      ])
    })
],
    
  providers: [XboxliveService],
  bootstrap: [AppComponent, MsalRedirectComponent]
})
export class AppModule {}
