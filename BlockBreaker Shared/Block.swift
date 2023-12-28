//
//  Block.swift
//  BlockBreaker
//

import SpriteKit

class Block: SKSpriteNode {
    var isSelected = false
    var coordinate: CGPoint?

    convenience init(color: SKColor, size: CGSize, coordinate: CGPoint) {
        self.init(texture: nil, color: color, size: size)
        self.coordinate = coordinate
    }
}
