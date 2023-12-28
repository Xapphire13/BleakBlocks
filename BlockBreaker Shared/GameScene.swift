//
//  GameScene.swift
//  BlockBreaker Shared
//

import SpriteKit

class GameScene: SKScene {
    var blocks: [[Block?]] = []
    let blockSize = CGSize(width: 50, height: 50)
    let GAME_SIZE = 5
    let COLORS = [SKColor.magenta, SKColor.green, SKColor.blue]

    class func newGameScene() -> GameScene {
        // Load 'GameScene.sks' as an SKScene.
        guard let scene = SKScene(fileNamed: "GameScene") as? GameScene else {
            print("Failed to load GameScene.sks")
            abort()
        }
        
        // Set the scale mode to scale to fit the window
        scene.scaleMode = .aspectFill
        
        return scene
    }

    override func didMove(to view: SKView) {
        self.setupGame()
    }

    func setupGame() {
        for row in 0..<GAME_SIZE {
            var rowBlocks: [Block] = []

            for col in 0..<GAME_SIZE {
                let color = COLORS[Int.random(in: 0..<COLORS.count)]
                
                let block = Block(
                    color: color,
                    size: blockSize,
                    coordinate: CGPoint(x: col, y: row)
                )
                block.position = CGPoint(x: col + col * Int(blockSize.width), y: row + row * Int(blockSize.height))
                block.name = "block\(row)_\(col)"
                addChild(block)

                rowBlocks.append(block)
            }

            blocks.append(rowBlocks)
        }
    }
    
    func removeBlock(_ block: Block) {
        // Animate removal
        let fadeOutAction = SKAction.fadeOut(withDuration: 0.5)
        let removeAction = SKAction.removeFromParent()
        block.run(SKAction.sequence([fadeOutAction, removeAction]))

        // Update game state (remove block from the array)
        if let rowIndex = blocks.firstIndex(where: { $0.contains(block) }) {
            if let colIndex = blocks[rowIndex].firstIndex(of: block) {
                blocks[rowIndex][colIndex] = nil
            }
        }
    }


    func selectAndRemoveBlocks(startingFrom block: Block) {
        let group = self.findGroup(block)
        group.forEach { block in
            self.removeBlock(block)
        }
    }
    
    func removeAndShiftBlocks(selectedBlocks: [Block]) {
        // Implement block removal and column shifting logic
    }
    
    func findGroup(_ block: Block) -> [Block] {
        var group: Set<Block> = []
        
        self.checkNeighbors(block, group: &group)
        
        return Array(group)
    }
    
    func checkNeighbors(_ block: Block, group: inout Set<Block>) {
        if (group.contains(block)) {
            return
        }
        
        group.insert(block)
        let (col, row) = (Int(block.coordinate.x), Int(block.coordinate.y))
        
        // Left
        if col >= 1 {
            if let leftBlock = self.blocks[row][col - 1] {
                if leftBlock.color == block.color {
                    self.checkNeighbors(leftBlock, group: &group)
                }
            }
        }
        
        // Right
        if col < GAME_SIZE - 1 {
            if let rightBlock = self.blocks[row][col + 1] {
                if rightBlock.color == block.color {
                    self.checkNeighbors(rightBlock, group: &group)
                }
            }
        }
        
        // Up
        if row < GAME_SIZE - 1 {
            if let upperBlock = self.blocks[row + 1][col] {
                if upperBlock.color == block.color {
                    self.checkNeighbors(upperBlock, group: &group)
                }
            }
        }
        
        // Down
        if row >= 1 {
            if let lowerBlock = self.blocks[row - 1][col] {
                if lowerBlock.color == block.color {
                    self.checkNeighbors(lowerBlock, group: &group)
                }
            }
        }
    }
}

#if os(iOS) || os(tvOS)
// Touch-based event handling
extension GameScene {
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        touches.forEach { touch in
            let location = touch.location(in: self)
            let node = atPoint(location)

            if let block = node as? Block {
                // Handle block selection and removal
                self.selectAndRemoveBlocks(startingFrom: block)
            }
        }
    }
}
#endif

#if os(OSX)
// Mouse-based event handling
extension GameScene {
    override func mouseUp(with event: NSEvent) {
        let location = event.location(in: self)
        let node = atPoint(location)

        if let block = node as? Block {
            // Handle block selection and removal
            self.selectAndRemoveBlocks(startingFrom: block)
        }
    }
    
    override func mouseEntered(with event: NSEvent) {
        let location = event.location(in: self)
        let node = atPoint(location)

        if let block = node as? Block {
            // Show hightlight on block and adjacent blocks of the same group
            let group = self.findGroup(block)
        }
    }
}
#endif

