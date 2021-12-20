const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let isMusl = false
let loadError = null

switch (platform) {
  case 'android':
    if (arch !== 'arm64') {
      throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'napi2.android-arm64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./napi2.android-arm64.node')
      } else {
        nativeBinding = require('napi2-android-arm64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'napi2.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./napi2.win32-x64-msvc.node')
          } else {
            nativeBinding = require('napi2-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'napi2.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./napi2.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('napi2-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'napi2.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./napi2.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('napi2-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'napi2.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./napi2.darwin-x64.node')
          } else {
            nativeBinding = require('napi2-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'napi2.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./napi2.darwin-arm64.node')
          } else {
            nativeBinding = require('napi2-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'napi2.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./napi2.freebsd-x64.node')
      } else {
        nativeBinding = require('napi2-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        isMusl = readFileSync('/usr/bin/ldd', 'utf8').includes('musl')
        if (isMusl) {
          localFileExisted = existsSync(
            join(__dirname, 'napi2.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./napi2.linux-x64-musl.node')
            } else {
              nativeBinding = require('napi2-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'napi2.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./napi2.linux-x64-gnu.node')
            } else {
              nativeBinding = require('napi2-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        isMusl = readFileSync('/usr/bin/ldd', 'utf8').includes('musl')
        if (isMusl) {
          localFileExisted = existsSync(
            join(__dirname, 'napi2.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./napi2.linux-arm64-musl.node')
            } else {
              nativeBinding = require('napi2-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'napi2.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./napi2.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('napi2-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        localFileExisted = existsSync(
          join(__dirname, 'napi2.linux-arm-gnueabihf.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./napi2.linux-arm-gnueabihf.node')
          } else {
            nativeBinding = require('napi2-linux-arm-gnueabihf')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { sum, getStr, getNums, sumNums, readFileAsync, asyncMultiTwo, bigAdd, createBigIntI64, createBigInt, getCurrentDir, readFile, throwError, ClassWithFactory, Kind, CustomNumEnum, enumToI32, mapOption, returnNull, returnUndefined, add, fibonacci, asyncPlus100 } = nativeBinding

module.exports.sum = sum
module.exports.getStr = getStr
module.exports.getNums = getNums
module.exports.sumNums = sumNums
module.exports.readFileAsync = readFileAsync
module.exports.asyncMultiTwo = asyncMultiTwo
module.exports.bigAdd = bigAdd
module.exports.createBigIntI64 = createBigIntI64
module.exports.createBigInt = createBigInt
module.exports.getCurrentDir = getCurrentDir
module.exports.readFile = readFile
module.exports.throwError = throwError
module.exports.ClassWithFactory = ClassWithFactory
module.exports.Kind = Kind
module.exports.CustomNumEnum = CustomNumEnum
module.exports.enumToI32 = enumToI32
module.exports.mapOption = mapOption
module.exports.returnNull = returnNull
module.exports.returnUndefined = returnUndefined
module.exports.add = add
module.exports.fibonacci = fibonacci
module.exports.asyncPlus100 = asyncPlus100
