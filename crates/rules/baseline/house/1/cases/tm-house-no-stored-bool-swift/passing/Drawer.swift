public struct Drawer {
    public var id: Int64
    public var title: String
    public var operationalBitmap: Int64 = 0
    public var isArchived: Bool {
        get { operationalBitmap & (1 << 3) != 0 }
        set { operationalBitmap = newValue ? operationalBitmap | (1 << 3) : operationalBitmap & ~(1 << 3) }
    }
}
