const std = @import("std");
// const crypto = @import("std/crypto");
const log = @import("log.zig");
const allocator = log.allocator;
const err = log.err;
const err_const = log.err_const;
const err_allocate = log.err_allocate;
const warn = log.warn;
const warn_const = log.warn_const;
const info = log.info;
const info_const = log.info_const;
const PtrToCompound = log.PtrToCompound;
pub const allocUint8 = log.allocUint8;

export fn prepareBufToSend(ofs: u32, timestamp: u32) void {
    const ptr: [*]u8 = @ptrFromInt(ofs);
    std.mem.writeInt(u32, ptr[0..@sizeOf(u32)], timestamp, .Little);
    ptr[@sizeOf(u32)] = 0;
}

export fn getFileId(ofs: u32, len: u32) u64 {
    const ptr: [*]u8 = @ptrFromInt(ofs);
    // const codecs = std.base64.standard;
    // const out_len = codecs.Encoder.calcSize(len);
    // const out_buf = allocUint8(out_len + 1);
    // const dest: []u8 = @constCast(out_buf[0..out_len]);
    const src: []u8 = ptr[0..len];
    var out: [16]u8 = undefined;
    // out[16] = 0;
    // const crypto_core = std.crypto.core;
    // _ = crypto_core;
    // const md5 = crypto.md5;
    // _ = md5;
    // _ = src;
    var h = std.crypto.hash.blake2.Blake2s128.init(.{});
    h.update(src);
    h.final(out[0..]);
    // const codecs = std.base64.standard;

    const url_safe_no_pad = std.base64.Codecs{
        .alphabet_chars = std.base64.url_safe_alphabet_chars,
        .pad_char = null,
        .decoderWithIgnore = urlSafeBase64DecoderWithIgnore,
        .Encoder = std.base64.Base64Encoder.init(std.base64.url_safe_alphabet_chars, null),
        .Decoder = std.base64.Base64Decoder.init(std.base64.url_safe_alphabet_chars, null),
    };
    const out_len = std.base64.url_safe_no_pad.Encoder.calcSize(16);
    const out_buf = allocUint8(out_len + 1);

    const dest: []u8 = @constCast(out_buf[0..out_len]);
    // const src: []u8 = ptr[0..len];
    _ = url_safe_no_pad.Encoder.encode(dest, &out);
    return PtrToCompound.from_str(dest).make();

    // return PtrToCompound.from_str(&out).make();
    // return 0;
}

fn urlSafeBase64DecoderWithIgnore(ignore: []const u8) std.base64.Base64DecoderWithIgnore {
    return std.base64.Base64DecoderWithIgnore.init(std.base64.url_safe_alphabet_chars, null, ignore);
}

// export fn encodeBase64(ofs: u32, len: u32) u64 {
//     const ptr: [*]u8 = @ptrFromInt(ofs);
//     const codecs = std.base64.standard;
//     const out_len = codecs.Encoder.calcSize(len);
//     const out_buf = allocUint8(out_len + 1);
//     const dest: []u8 = @constCast(out_buf[0..out_len]);
//     const src: []u8 = ptr[0..len];
//     _ = codecs.Encoder.encode(dest, src);
//     return PtrToCompound.from_str(dest).make();
// }

// export fn applyAlpha(alpha: u8, ofs: u32, len: u32) void {
//     const ptr: [*]u8 = @ptrFromInt(ofs);
//     var i: u32 = 0;
//     while (i < len) : (i += 4) {
//         ptr[i + 3] = @as(u8, @intCast(@as(u32, ptr[i + 3]) * @as(u32, alpha) / @as(u32, 255)));
//     }
// }
