import {Game, Difficulty, BlockType, Dir} from "../pkg/pacman.js";
import {memory} from "../pkg/pacman_bg.wasm";

export class PacManGame {
    static tickRate = 5;
    static renderRate = 20;
    static blockSize = 50;
    constructor(width, height, diff) {
        this.width = width;
        this.height = height;
        this.game = Game.new(width,height,diff);

        const validKeys = [
                "ArrowRight", "KeyD",
                "ArrowLeft", "KeyA",
                "ArrowUp", "KeyW",
                "ArrowDown", "KeyS"
        ];

        const setKey = (event) => {
            if (validKeys.includes(event.code)) 
                this.setDir(event.code);
        }

        document.addEventListener("keypress" , (event) => {setKey(event);});
        document.addEventListener("keydown" , (event) => {setKey(event);});
    }


    getGhosts() {
        const positions = ["y","x","prev_y","prev_x","color","prev_dir","vulnerable"];
        let parsed = [];
        let ghosts = this.game.ghosts();
        for (let i = 0; i < ghosts.length; i += positions.length) {
            let curr_obj = {};
            for (let j = 0; j < positions.length; ++j) 
                curr_obj[positions[j]] = ghosts[i + j];

            parsed.push(curr_obj);
        }
        return parsed; 
    }

    getPacman() {
        const positions = ["y","x","prev_y","prev_x","prev_dir","lives"];
        let p = this.game.pacman();
        let curr_obj = {};
        for (let j = 0; j < positions.length; ++j) 
            curr_obj[positions[j]] = p[j];
        return curr_obj; 
    }

    static dirToTup = (dir) => {
        switch(dir) {
            case Dir.Down: return [1,0];
            case Dir.Up: return [-1,0];
            case Dir.Left: return [0,-1];
            case Dir.Right: return [0,1];
            default: return [0,0];
        };
    }

    toCord(i) {
        return {y: Math.floor(i / this.width), x: i % this.width};
    }

    fieldAt(y,x) {
        return this.game.field_at(y,x);
    }
    field() {
        return this.game.field()
    }

    setDir(code) {
        console.log(code);
        this.game.set_dir(code);
    }
    getScore() {
        return this.game.score();
    }
    getLives() {
        return this.game.lives();
    }
}


export class GameRenderer {
    static RAD = Math.PI/180;
    static img = new Image();
    static spriteSize = 14;
    static canvas;
    constructor(game, tickRate, renderRate) {
        this.ctx = GameRenderer.canvas.getContext("2d");
        this.ctx.fillStyle = "#FFFF00";
        this.ctx.fillRect(0,0,50,50);
        this.game = game;
        this.tickRate = tickRate;
        this.renderRate = renderRate;
        this.renderTick = 0;

        GameRenderer.img.src = "../static/pacman_sprites.png";
        this.blockSize = Math.floor(Math.min(window.screen.width, window.screen.height) / Math.max(this.game.height, this.game.width));
        this.drawOffset = this.blockSize / 2;
        GameRenderer.canvas.height = game.height * this.blockSize + this.blockSize;
        GameRenderer.canvas.width = game.width * this.blockSize + this.blockSize;

    }

    #getCtxPos(i) {
        return i * this.blockSize + this.drawOffset;
    }

    #drawFieldAt(y,x) {
        const blockType = this.game.fieldAt(y,x);
        const halfBlock = Math.floor(this.blockSize / 2);
        this.ctx.beginPath();
        this.ctx.fillStyle = this.ctx.strokeStyle = "#FFB8AE";

        switch (blockType) {
            case BlockType.Wall : {
                this.ctx.strokeStyle = "#FF0000";
                [[-1,0],[1,0],[0,-1],[0,1]].forEach(([dy, dx]) => {
                    //if the current Block is the first or last block or has no neighbour in the current direction
                    if (y + dy < 0 || y + dy > this.game.height - 1 || x + dx < 0 || x + dx > this.game.width - 1 || this.game.fieldAt(y + dy, x + dx) != BlockType.Wall) {
                        let [begin_y, begin_x] = [(dy > 0 ? y + 1 : y), (dx > 0 ? x + 1 : x)];

                        let [end_y, end_x] = [begin_y + (dx != 0), begin_x + (dy != 0)];
                        this.ctx.moveTo(this.#getCtxPos(begin_x), this.#getCtxPos(begin_y));
                        this.ctx.lineTo(this.#getCtxPos(end_x), this.#getCtxPos(end_y));
                    }
                });
            break; } case BlockType.PacDot : {
                let dotSize = Math.ceil(this.blockSize / 10);
                this.ctx.fillRect(this.#getCtxPos(x) + halfBlock - 2, this.#getCtxPos(y) + halfBlock - 2, dotSize, dotSize);
            break; } case BlockType.PowerPellet : {
                let r = Math.ceil(this.blockSize / 5);
                this.ctx.arc(this.#getCtxPos(x) + halfBlock, this.#getCtxPos(y) + halfBlock, r, 0, Math.PI * 2);
                this.ctx.fill();
            break; } case BlockType.Gate : {
                this.ctx.lineWidth = Math.ceil(this.blockSize / 5);
                this.ctx.moveTo(this.#getCtxPos(x), this.#getCtxPos(y) + halfBlock);
                this.ctx.lineTo(this.#getCtxPos(x) + this.blockSize, this.#getCtxPos(y) + halfBlock);
                this.ctx.lineWidth = 1;
            }
        }
        this.ctx.stroke();
    }
    
    drawField() {
        this.ctx.beginPath();
        const fieldptr = this.game.field();
        const field = new Uint8Array(memory.buffer, fieldptr, this.game.width * this.game.height);

        for (let i = 0; i < field.length; ++i) 
            this.#drawFieldAt(this.game.toCord(i).y, this.game.toCord(i).x);
        
        this.ctx.stroke();
    }

    #getSpritePos = (i) => {
        return i * GameRenderer.spriteSize + 1 + i * 2;
    }

    #paintImg = (soy, sox,py,px) => {
        this.ctx.drawImage(GameRenderer.img, this.#getSpritePos(sox), this.#getSpritePos(soy), GameRenderer.spriteSize, GameRenderer.spriteSize, this.#getCtxPos(px) + 2, this.#getCtxPos(py) + 2, this.blockSize - 4, this.blockSize - 4);
    }

    #rotateAndPaintImg = (soy, sox,py,px,angle) => {
        this.ctx.save();
        this.ctx.translate(this.#getCtxPos(px) + this.blockSize / 2 , this.#getCtxPos(py) + this.blockSize / 2);
        this.ctx.rotate(angle * GameRenderer.RAD);
        this.ctx.translate(-(this.blockSize - 4)/2,-(this.blockSize - 4) / 2 );
        this.ctx.drawImage(GameRenderer.img, this.#getSpritePos(sox), this.#getSpritePos(soy), GameRenderer.spriteSize, GameRenderer.spriteSize, 0, 0, this.blockSize - 4, this.blockSize - 4);
        this.ctx.restore();
    }
    #getTickAdjustedCord(obj, subtick) {
        let off = subtick * (this.tickRate/this.renderRate);
        let [off_y, off_x] = PacManGame.dirToTup(obj.prev_dir);
        off_y *= off; off_x *= off;
        return [obj.prev_y + off_y, obj.prev_x + off_x];
    }

    #drawPacman(subtick) {
        let p = this.game.getPacman();

        let sprite_offset = 2 - this.renderTick%3;
        let rotation = p.prev_dir * 90;
        if (p.prev_dir == Dir.None) { 
            sprite_offset = 2;
        }

        let [y,x] = this.#getTickAdjustedCord(p, subtick);
        if (this.prevPacX !== undefined && this.prevPacY !== undefined) 
            this.ctx.clearRect(this.#getCtxPos(this.prevPacX) + 1, this.#getCtxPos(this.prevPacY) + 1, this.blockSize - 2, this.blockSize - 2);

        this.#rotateAndPaintImg(0,sprite_offset,y,x,rotation);
        this.prevPacX = x;
        this.prevPacY = y;

        let ghosts_behind = this.game.getGhosts().filter((ghost) => ghost.x == p.prev_x && ghost.prev_x != p.x && ghost.y == p.prev_y && ghost.prev_y != p.y);
        ghosts_behind.forEach((ghost) => this.#drawGhost(ghost,subtick));
    }

    #drawGhost(ghost, subtick) {
        if (this.prevGhostPos.length == 4){ 
            let prevPos = this.prevGhostPos.shift();
            this.ctx.clearRect(this.#getCtxPos(prevPos.x) + 1, this.#getCtxPos(prevPos.y) + 1, this.blockSize - 2, this.blockSize - 2);
        }

        let p = this.game.getPacman();


        this.#drawFieldAt(ghost.prev_y, ghost.prev_x);
        this.#drawFieldAt(ghost.y, ghost.x);

        let soy = ghost.vulnerable ? 5 : ghost.color;
        let sox = this.renderTick % 2 == 0;
        sox += ghost.vulnerable ? 0 : ghost.prev_dir == Dir.None ? 6 : ghost.prev_dir * 2;

        let [y,x] = this.#getTickAdjustedCord(ghost, subtick);
        this.prevGhostPos.push({"y": y, "x": x});
        this.#paintImg(soy,sox,y, x);

        //if pacman was behin the ghost he would be hidden by the above clearRect call
        if (p.y == ghost.prev_y && p.x == ghost.prev_x) this.#drawPacman(subtick);
        let ghosts_behind = this.game.getGhosts().filter((g) => g.x == ghost.prev_x && g.prev_x != ghost.x && g.y == ghost.prev_y && g.prev_y != g.prev_x);
        ghosts_behind.forEach((ghost) => this.#drawGhost(ghost,subtick));
    }

    #drawGhosts(subtick) {
        if (!this.prevGhostPos) this.prevGhostPos = [];
        this.game.getGhosts().forEach((ghost) => this.#drawGhost(ghost, subtick));
    }

    renderEntities(subtick) {
        ++this.renderTick;
        this.#drawPacman(subtick);
        this.#drawGhosts(subtick);
    }

    clearAll() {
        this.ctx.clearRect(0,0,getCtxPos(this.width),getCtxPos(this.height));
    }
}

const score = document.getElementById("score");
const lives = document.getElementById("lives");

let game = new PacManGame(31,31,Difficulty.Normal);

GameRenderer.canvas = document.getElementById("game-canvas");
let renderer = new GameRenderer(game,PacManGame.tickRate,PacManGame.renderRate);

function timeout(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function tick() {
    let gameover = game.game.tick();
    score.innerText = "Score: " + game.getScore();
    lives.innerText = "Lives: " + game.getLives();

    for(let subtick = 1; subtick <= PacManGame.renderRate / PacManGame.tickRate; ++subtick) {
        await timeout(1000 / PacManGame.renderRate);
        renderer.renderEntities(subtick);
    }

    if (gameover) return;
    requestAnimationFrame(tick);
}

renderer.drawField();
requestAnimationFrame(tick);
