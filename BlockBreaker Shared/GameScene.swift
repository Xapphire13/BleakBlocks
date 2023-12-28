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
        let trackingArea = NSTrackingArea(
            rect: view.frame,
            options: [.activeInKeyWindow, .mouseMoved],
            owner: self,
            userInfo: nil
        )
        view.addTrackingArea(trackingArea)
    }

    func setupGame() {
        for row in 0..<GAME_SIZE {
            var rowBlocks: [Block] = []

            for col in 0..<GAME_SIZE {
                let color = COLORS[Int.random(in: 0..<COLORS.count)]
                
                let block = Block(
                    color: color,
                    size: blockSize
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
        block.removeFromParent()

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
    
    func shiftBlocks() {
        var areBlocksFalling = false
        for col in 0..<GAME_SIZE {
            var emptyIndex: Int? = nil
            for row in 0..<GAME_SIZE {
                if self.blocks[row][col] == nil {
                    emptyIndex = row
                    break
                }
            }
            
            if var emptyIndex = emptyIndex {
                for row in emptyIndex+1..<GAME_SIZE {
                    if let block = self.blocks[row][col] {
                        areBlocksFalling = true
                        block.run(SKAction.moveTo(y: CGFloat(emptyIndex) * 50 + CGFloat(emptyIndex), duration: 0.2))
                        self.blocks[row][col] = nil
                        self.blocks[emptyIndex][col] = block
                        
                        emptyIndex += 1
                    }
                }
            }
        }
        
        run(SKAction.wait(forDuration: areBlocksFalling ? 0.2 : 0.0)) {
            for leftCol in 0..<self.GAME_SIZE {
                if self.blocks[0][leftCol] == nil {
                    for rightCol in leftCol+1..<self.GAME_SIZE {
                        if self.blocks[0][rightCol] != nil {
                            for row in 0..<self.GAME_SIZE {
                                if let block = self.blocks[row][rightCol] {
                                    block.run(SKAction.moveTo(x: CGFloat(leftCol) * 50 + CGFloat(leftCol), duration: 0.2))
                                    self.blocks[row][rightCol] = nil
                                    self.blocks[row][leftCol] = block
                                }
                            }
                            break
                        }
                    }
                }
            }
        }
    }
    
    func findGroup(_ block: Block) -> [Block] {
        var group: Set<Block> = []
        
        self.checkNeighbors(block, group: &group)
        
        return Array(group)
    }
    
    func findCoordinate(_ block: Block) -> CGPoint {
        if let rowIndex = blocks.firstIndex(where: { $0.contains(block) }) {
            if let colIndex = blocks[rowIndex].firstIndex(of: block) {
                return CGPoint(x: rowIndex, y: colIndex)
            }
        }
        
        return CGPoint()
    }
    
    func checkNeighbors(_ block: Block, group: inout Set<Block>) {
        if (group.contains(block)) {
            return
        }
        
        group.insert(block)
        let coordinate = self.findCoordinate(block)
        let (row, col) =  (Int(coordinate.x), Int(coordinate.y))
        
        // Left
        if col >= 1 {
            if let leftBlock = self.blocks[row][col - 1] {
                if leftBlock.originalColor == block.originalColor {
                    self.checkNeighbors(leftBlock, group: &group)
                }
            }
        }
        
        // Right
        if col < GAME_SIZE - 1 {
            if let rightBlock = self.blocks[row][col + 1] {
                if rightBlock.originalColor == block.originalColor {
                    self.checkNeighbors(rightBlock, group: &group)
                }
            }
        }
        
        // Up
        if row < GAME_SIZE - 1 {
            if let upperBlock = self.blocks[row + 1][col] {
                if upperBlock.originalColor == block.originalColor {
                    self.checkNeighbors(upperBlock, group: &group)
                }
            }
        }
        
        // Down
        if row >= 1 {
            if let lowerBlock = self.blocks[row - 1][col] {
                if lowerBlock.originalColor == block.originalColor {
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
            self.shiftBlocks()
            
            var isGameOver = true
            for row in self.blocks {
                for block in row {
                    if block != nil {
                        isGameOver = false
                        break
                    }
                }
                
                if !isGameOver {
                    break;
                }
            }
            
            if isGameOver {
                self.blocks = []
                self.setupGame()
            }
        }
    }
    
//    override func mouseMoved(with event: NSEvent) {
//        let location = event.location(in: self)
//        let node = atPoint(location)
//        
//        if let block = node as? Block {
//            let group = self.findGroup(block)
//            group.forEach { block in
//                block.highlight()
//            }
//        } else {
//            self.blocks.forEach { row in
//                row.forEach { block in
//                    block?.unhighlight()
//                }
//            }
//        }
//    }
}
#endif

