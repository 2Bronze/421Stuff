import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'player-card',
  templateUrl: './player-card.component.html',
  styleUrls: ['./player-card.component.css']
})
export class PlayerCardComponent implements OnInit {

  player_name = "Player";
  wins = 0;
  losses = 0;
  wr = 0;

  constructor() { }

  ngOnInit(): void {
  }

}
