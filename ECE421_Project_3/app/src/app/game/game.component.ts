import { Component, OnInit } from '@angular/core';
import * as wasm from "rust-api";

@Component({
  selector: 'app-game',
  templateUrl: './game.component.html',
  styleUrls: ['./game.component.css']
})
export class GameComponent implements OnInit {
  game_board = new Array(6).fill(new Array(7).fill(0)); //7cols, 6rows
  constructor() { }

  ngOnInit(): void {
  }

}
