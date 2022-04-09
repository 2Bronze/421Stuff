import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { HttpClientModule } from '@angular/common/http';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { MainModalComponent } from './main-modal/main-modal.component';
import { PlayerCardComponent } from './player-card/player-card.component';
import { GameComponent } from './game/game.component';
import { MatchHistoryComponent } from './match-history/match-history.component';
import { RecentWinsComponent } from './recent-wins/recent-wins.component';

@NgModule({
  declarations: [
    AppComponent,
    MainModalComponent,
    PlayerCardComponent,
    GameComponent,
    MatchHistoryComponent,
    RecentWinsComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    HttpClientModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
