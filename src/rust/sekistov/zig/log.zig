pub extern fn console_info(compound: u64) void;
pub extern fn console_warn(compound: u64) void;
pub extern fn console_err(compound: u64) void;
pub extern fn console_err_allocate(size: u32, compound: u64) void;

const std = @import("std");

pub const allocator = std.heap.wasm_allocator;

pub fn info(str:[]u8) void {
    const ptr_to_compound = PtrToCompound {
        .ptr = str.ptr,
        .len = str.len,
    };
    console_info(ptr_to_compound.make());
}

pub fn info_const(str:[]const u8) void {
    const ptr_to_compound = PtrToCompound {
        .ptr = @constCast(str.ptr),
        .len = str.len,
        .is_const = true,
    };
    console_info(ptr_to_compound.make());
}

pub fn warn(str:[]u8) void {
    const ptr_to_compound = PtrToCompound {
        .ptr = str.ptr,
        .len = str.len,
    };
    console_warn(ptr_to_compound.make());
}

pub fn warn_const(str:[]const u8) void {
    const ptr_to_compound = PtrToCompound {
        .ptr = @constCast(str.ptr),
        .len = str.len,
        .is_const = true,
    };
    console_warn(ptr_to_compound.make());
}

pub fn err(str:[]u8) void {
    const ptr_to_compound = PtrToCompound {
        .ptr = str.ptr,
        .len = str.len,
    };
    console_err(ptr_to_compound.make());
}

pub fn err_const(str:[]const u8) void {
    const ptr_to_compound = PtrToCompound {
        .ptr = @constCast(str.ptr),
        .len = str.len,
        .is_const = true,
    };
    console_err(ptr_to_compound.make());
}

pub fn err_allocate(size: u32, str:[]const u8) void {
    const ptr_to_compound = PtrToCompound {
        .ptr = @constCast(str.ptr),
        .len = str.len,
        .is_const = true,
    };
    console_err_allocate(size, ptr_to_compound.make());
}


pub const PtrToCompound = struct {
    ptr: [*]u8,
    len: u32,
    is_const: bool = false,
    is_str: bool = true,

    pub fn from_str(str: []u8) PtrToCompound {
        return .{
            .ptr = str.ptr,
            .len = str.len,
        };
    }
    fn decode(compound: u64) PtrToCompound {
        return .{
            .ptr = @ptrFromInt(@as(u32, @intCast(compound & 0xFFFFFFFF))),
            .len = @as(u32, @intCast(compound >> 34)),
            .is_const = compound & (1 << 33) != 0,
            .is_str = compound & (1 << 32) != 0
        };
    }
    pub fn make(self: PtrToCompound) u64 {
        var ret: u64 = @intCast(self.len);
        ret <<= 34;
        if (self.is_const) {
            ret |= (1 << 33);
        }
        if (self.is_str) {
            ret |= (1 << 32);
        }
        ret |= @intFromPtr(self.ptr);
        return ret;
    }
    fn free(self: PtrToCompound) void {
        if (!self.is_const) {
            if (!self.is_str) {
                freeUint8(self.ptr, self.len);
            } else {
                freeUint8(self.ptr, self.len + 1);
            }
        }
    }
};

export fn freeCompound(compound: u64) void {
    const ptr_to_compound = PtrToCompound.decode(compound);
    ptr_to_compound.free();
}

export fn freeUint8(ptr: [*]u8, len: usize) void {
    allocator.free(ptr[0..len]);
}

pub export fn allocUint8(length: usize) [*]const u8 {
    const slice = allocator.alloc(u8, length) catch {
        err_allocate(length, "allocUint8");
        @panic("failed to allocate memory");
    };
    return slice.ptr;
}
