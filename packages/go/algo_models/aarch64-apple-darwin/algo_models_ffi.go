
package aarch64-apple-darwin

// #include <algo_models_ffi.h>
import "C"

import (
	"bytes"
	"fmt"
	"io"
	"unsafe"
	"encoding/binary"
	"math"
)



// This is needed, because as of go 1.24
// type RustBuffer C.RustBuffer cannot have methods,
// RustBuffer is treated as non-local type
type GoRustBuffer struct {
	inner C.RustBuffer
}

type RustBufferI interface {
	AsReader() *bytes.Reader
	Free()
	ToGoBytes() []byte
	Data() unsafe.Pointer
	Len() uint64
	Capacity() uint64
}

func RustBufferFromExternal(b RustBufferI) GoRustBuffer {
	return GoRustBuffer {
		inner: C.RustBuffer {
			capacity: C.uint64_t(b.Capacity()),
			len: C.uint64_t(b.Len()),
			data: (*C.uchar)(b.Data()),
		},
	}
}

func (cb GoRustBuffer) Capacity() uint64 {
	return uint64(cb.inner.capacity)
}

func (cb GoRustBuffer) Len() uint64 {
	return uint64(cb.inner.len)
}

func (cb GoRustBuffer) Data() unsafe.Pointer {
	return unsafe.Pointer(cb.inner.data)
}

func (cb GoRustBuffer) AsReader() *bytes.Reader {
	b := unsafe.Slice((*byte)(cb.inner.data), C.uint64_t(cb.inner.len))
	return bytes.NewReader(b)
}

func (cb GoRustBuffer) Free() {
	rustCall(func( status *C.RustCallStatus) bool {
		C.ffi_algo_models_ffi_rustbuffer_free(cb.inner, status)
		return false
	})
}

func (cb GoRustBuffer) ToGoBytes() []byte {
	return C.GoBytes(unsafe.Pointer(cb.inner.data), C.int(cb.inner.len))
}


func stringToRustBuffer(str string) C.RustBuffer {
	return bytesToRustBuffer([]byte(str))
}

func bytesToRustBuffer(b []byte) C.RustBuffer {
	if len(b) == 0 {
		return C.RustBuffer{}
	}
	// We can pass the pointer along here, as it is pinned
	// for the duration of this call
	foreign := C.ForeignBytes {
		len: C.int(len(b)),
		data: (*C.uchar)(unsafe.Pointer(&b[0])),
	}
	
	return rustCall(func( status *C.RustCallStatus) C.RustBuffer {
		return C.ffi_algo_models_ffi_rustbuffer_from_bytes(foreign, status)
	})
}


type BufLifter[GoType any] interface {
	Lift(value RustBufferI) GoType
}

type BufLowerer[GoType any] interface {
	Lower(value GoType)C.RustBuffer
}

type BufReader[GoType any] interface {
	Read(reader io.Reader) GoType
}

type BufWriter[GoType any] interface {
	Write(writer io.Writer, value GoType)
}

func LowerIntoRustBuffer[GoType any](bufWriter BufWriter[GoType], value GoType) C.RustBuffer {
	// This might be not the most efficient way but it does not require knowing allocation size
	// beforehand
	var buffer bytes.Buffer
	bufWriter.Write(&buffer, value)

	bytes, err := io.ReadAll(&buffer)
	if err != nil {
		panic(fmt.Errorf("reading written data: %w", err))
	}
	return bytesToRustBuffer(bytes)
}

func LiftFromRustBuffer[GoType any](bufReader BufReader[GoType], rbuf RustBufferI) GoType {
	defer rbuf.Free()
	reader := rbuf.AsReader()
	item := bufReader.Read(reader)
	if reader.Len() > 0 {
		// TODO: Remove this
		leftover, _ := io.ReadAll(reader)
		panic(fmt.Errorf("Junk remaining in buffer after lifting: %s", string(leftover)))
	}
	return item
}



func rustCallWithError[E any, U any](converter BufReader[*E], callback func(*C.RustCallStatus) U) (U, *E) {
	var status C.RustCallStatus
	returnValue := callback(&status)
	err := checkCallStatus(converter, status)
	return returnValue, err
}

func checkCallStatus[E any](converter BufReader[*E], status C.RustCallStatus) *E {
	switch status.code {
	case 0:
		return nil
	case 1:
		return LiftFromRustBuffer(converter, GoRustBuffer { inner: status.errorBuf })
	case 2:
		// when the rust code sees a panic, it tries to construct a rustBuffer
		// with the message.  but if that code panics, then it just sends back
		// an empty buffer.
		if status.errorBuf.len > 0 {
			panic(fmt.Errorf("%s", FfiConverterStringINSTANCE.Lift(GoRustBuffer { inner: status.errorBuf })))
		} else {
			panic(fmt.Errorf("Rust panicked while handling Rust panic"))
		}
	default:
		panic(fmt.Errorf("unknown status code: %d", status.code))
	}
}

func checkCallStatusUnknown(status C.RustCallStatus) error {
	switch status.code {
	case 0:
		return nil
	case 1:
		panic(fmt.Errorf("function not returning an error returned an error"))
	case 2:
		// when the rust code sees a panic, it tries to construct aC.RustBuffer
		// with the message.  but if that code panics, then it just sends back
		// an empty buffer.
		if status.errorBuf.len > 0 {
			panic(fmt.Errorf("%s", FfiConverterStringINSTANCE.Lift(GoRustBuffer {
				inner: status.errorBuf,
			})))
		} else {
			panic(fmt.Errorf("Rust panicked while handling Rust panic"))
		}
	default:
		return fmt.Errorf("unknown status code: %d", status.code)
	}
}

func rustCall[U any](callback func(*C.RustCallStatus) U) U {
	returnValue, err := rustCallWithError[error](nil, callback)
	if err != nil {
		panic(err)
	}
	return returnValue
}

type NativeError interface {
	AsError() error
}


func writeInt8(writer io.Writer, value int8) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint8(writer io.Writer, value uint8) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt16(writer io.Writer, value int16) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint16(writer io.Writer, value uint16) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt32(writer io.Writer, value int32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint32(writer io.Writer, value uint32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeInt64(writer io.Writer, value int64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeUint64(writer io.Writer, value uint64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeFloat32(writer io.Writer, value float32) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}

func writeFloat64(writer io.Writer, value float64) {
	if err := binary.Write(writer, binary.BigEndian, value); err != nil {
		panic(err)
	}
}


func readInt8(reader io.Reader) int8 {
	var result int8
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint8(reader io.Reader) uint8 {
	var result uint8
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt16(reader io.Reader) int16 {
	var result int16
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint16(reader io.Reader) uint16 {
	var result uint16
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt32(reader io.Reader) int32 {
	var result int32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint32(reader io.Reader) uint32 {
	var result uint32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readInt64(reader io.Reader) int64 {
	var result int64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readUint64(reader io.Reader) uint64 {
	var result uint64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readFloat32(reader io.Reader) float32 {
	var result float32
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func readFloat64(reader io.Reader) float64 {
	var result float64
	if err := binary.Read(reader, binary.BigEndian, &result); err != nil {
		panic(err)
	}
	return result
}

func init() {
        
        uniffiCheckChecksums()
}


func uniffiCheckChecksums() {
	// Get the bindings contract version from our ComponentInterface
	bindingsContractVersion := 26
	// Get the scaffolding contract version by calling the into the dylib
	scaffoldingContractVersion := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint32_t {
		return C.ffi_algo_models_ffi_uniffi_contract_version()
	})
	if bindingsContractVersion != int(scaffoldingContractVersion) {
		// If this happens try cleaning and rebuilding your project
		panic("algo_models_ffi: UniFFI contract version mismatch")
	}
	{
	checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_algo_models_ffi_checksum_func_address_from_pub_key()
	})
	if checksum != 7003 {
		// If this happens try cleaning and rebuilding your project
		panic("algo_models_ffi: uniffi_algo_models_ffi_checksum_func_address_from_pub_key: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_algo_models_ffi_checksum_func_address_from_string()
	})
	if checksum != 44254 {
		// If this happens try cleaning and rebuilding your project
		panic("algo_models_ffi: uniffi_algo_models_ffi_checksum_func_address_from_string: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_algo_models_ffi_checksum_func_attach_signature()
	})
	if checksum != 54550 {
		// If this happens try cleaning and rebuilding your project
		panic("algo_models_ffi: uniffi_algo_models_ffi_checksum_func_attach_signature: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_algo_models_ffi_checksum_func_decode_transaction()
	})
	if checksum != 42819 {
		// If this happens try cleaning and rebuilding your project
		panic("algo_models_ffi: uniffi_algo_models_ffi_checksum_func_decode_transaction: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_algo_models_ffi_checksum_func_encode_transaction()
	})
	if checksum != 30175 {
		// If this happens try cleaning and rebuilding your project
		panic("algo_models_ffi: uniffi_algo_models_ffi_checksum_func_encode_transaction: UniFFI API checksum mismatch")
	}
	}
	{
	checksum := rustCall(func(_uniffiStatus *C.RustCallStatus) C.uint16_t {
		return C.uniffi_algo_models_ffi_checksum_func_get_encoded_transaction_type()
	})
	if checksum != 10970 {
		// If this happens try cleaning and rebuilding your project
		panic("algo_models_ffi: uniffi_algo_models_ffi_checksum_func_get_encoded_transaction_type: UniFFI API checksum mismatch")
	}
	}
}




type FfiConverterUint64 struct{}

var FfiConverterUint64INSTANCE = FfiConverterUint64{}

func (FfiConverterUint64) Lower(value uint64) C.uint64_t {
	return C.uint64_t(value)
}

func (FfiConverterUint64) Write(writer io.Writer, value uint64) {
	writeUint64(writer, value)
}

func (FfiConverterUint64) Lift(value C.uint64_t) uint64 {
	return uint64(value)
}

func (FfiConverterUint64) Read(reader io.Reader) uint64 {
	return readUint64(reader)
}

type FfiDestroyerUint64 struct {}

func (FfiDestroyerUint64) Destroy(_ uint64) {}


type FfiConverterString struct{}

var FfiConverterStringINSTANCE = FfiConverterString{}

func (FfiConverterString) Lift(rb RustBufferI) string {
	defer rb.Free()
	reader := rb.AsReader()
	b, err := io.ReadAll(reader)
	if err != nil {
		panic(fmt.Errorf("reading reader: %w", err))
	}
	return string(b)
}

func (FfiConverterString) Read(reader io.Reader) string {
	length := readInt32(reader)
	buffer := make([]byte, length)
	read_length, err := reader.Read(buffer)
	if err != nil {
		panic(err)
	}
	if read_length != int(length) {
		panic(fmt.Errorf("bad read length when reading string, expected %d, read %d", length, read_length))
	}
	return string(buffer)
}

func (FfiConverterString) Lower(value string) C.RustBuffer {
	return stringToRustBuffer(value)
}

func (FfiConverterString) Write(writer io.Writer, value string) {
	if len(value) > math.MaxInt32 {
		panic("String is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	write_length, err := io.WriteString(writer, value)
	if err != nil {
		panic(err)
	}
	if write_length != len(value) {
		panic(fmt.Errorf("bad write length when writing string, expected %d, written %d", len(value), write_length))
	}
}

type FfiDestroyerString struct {}

func (FfiDestroyerString) Destroy(_ string) {}


type FfiConverterBytes struct{}

var FfiConverterBytesINSTANCE = FfiConverterBytes{}

func (c FfiConverterBytes) Lower(value []byte) C.RustBuffer {
	return LowerIntoRustBuffer[[]byte](c, value)
}

func (c FfiConverterBytes) Write(writer io.Writer, value []byte) {
	if len(value) > math.MaxInt32 {
		panic("[]byte is too large to fit into Int32")
	}

	writeInt32(writer, int32(len(value)))
	write_length, err := writer.Write(value)
	if err != nil {
		panic(err)
	}
	if write_length != len(value) {
		panic(fmt.Errorf("bad write length when writing []byte, expected %d, written %d", len(value), write_length))
	}
}

func (c FfiConverterBytes) Lift(rb RustBufferI) []byte {
	return LiftFromRustBuffer[[]byte](c, rb)
}

func (c FfiConverterBytes) Read(reader io.Reader) []byte {
	length := readInt32(reader)
	buffer := make([]byte, length)
	read_length, err := reader.Read(buffer)
	if err != nil {
		panic(err)
	}
	if read_length != int(length) {
		panic(fmt.Errorf("bad read length when reading []byte, expected %d, read %d", length, read_length))
	}
	return buffer
}

type FfiDestroyerBytes struct {}

func (FfiDestroyerBytes) Destroy(_ []byte) {}



type Address struct {
	Address string
	PubKey ByteBuf
}

func (r *Address) Destroy() {
		FfiDestroyerString{}.Destroy(r.Address);
		FfiDestroyerTypeByteBuf{}.Destroy(r.PubKey);
}

type FfiConverterAddress struct {}

var FfiConverterAddressINSTANCE = FfiConverterAddress{}

func (c FfiConverterAddress) Lift(rb RustBufferI) Address {
	return LiftFromRustBuffer[Address](c, rb)
}

func (c FfiConverterAddress) Read(reader io.Reader) Address {
	return Address {
			FfiConverterStringINSTANCE.Read(reader),
			FfiConverterTypeByteBufINSTANCE.Read(reader),
	}
}

func (c FfiConverterAddress) Lower(value Address) C.RustBuffer {
	return LowerIntoRustBuffer[Address](c, value)
}

func (c FfiConverterAddress) Write(writer io.Writer, value Address) {
		FfiConverterStringINSTANCE.Write(writer, value.Address);
		FfiConverterTypeByteBufINSTANCE.Write(writer, value.PubKey);
}

type FfiDestroyerAddress struct {}

func (_ FfiDestroyerAddress) Destroy(value Address) {
	value.Destroy()
}


type AssetTransferTransactionFields struct {
	AssetId uint64
	Amount uint64
	Receiver Address
	AssetSender *Address
	CloseRemainderTo *Address
}

func (r *AssetTransferTransactionFields) Destroy() {
		FfiDestroyerUint64{}.Destroy(r.AssetId);
		FfiDestroyerUint64{}.Destroy(r.Amount);
		FfiDestroyerAddress{}.Destroy(r.Receiver);
		FfiDestroyerOptionalAddress{}.Destroy(r.AssetSender);
		FfiDestroyerOptionalAddress{}.Destroy(r.CloseRemainderTo);
}

type FfiConverterAssetTransferTransactionFields struct {}

var FfiConverterAssetTransferTransactionFieldsINSTANCE = FfiConverterAssetTransferTransactionFields{}

func (c FfiConverterAssetTransferTransactionFields) Lift(rb RustBufferI) AssetTransferTransactionFields {
	return LiftFromRustBuffer[AssetTransferTransactionFields](c, rb)
}

func (c FfiConverterAssetTransferTransactionFields) Read(reader io.Reader) AssetTransferTransactionFields {
	return AssetTransferTransactionFields {
			FfiConverterUint64INSTANCE.Read(reader),
			FfiConverterUint64INSTANCE.Read(reader),
			FfiConverterAddressINSTANCE.Read(reader),
			FfiConverterOptionalAddressINSTANCE.Read(reader),
			FfiConverterOptionalAddressINSTANCE.Read(reader),
	}
}

func (c FfiConverterAssetTransferTransactionFields) Lower(value AssetTransferTransactionFields) C.RustBuffer {
	return LowerIntoRustBuffer[AssetTransferTransactionFields](c, value)
}

func (c FfiConverterAssetTransferTransactionFields) Write(writer io.Writer, value AssetTransferTransactionFields) {
		FfiConverterUint64INSTANCE.Write(writer, value.AssetId);
		FfiConverterUint64INSTANCE.Write(writer, value.Amount);
		FfiConverterAddressINSTANCE.Write(writer, value.Receiver);
		FfiConverterOptionalAddressINSTANCE.Write(writer, value.AssetSender);
		FfiConverterOptionalAddressINSTANCE.Write(writer, value.CloseRemainderTo);
}

type FfiDestroyerAssetTransferTransactionFields struct {}

func (_ FfiDestroyerAssetTransferTransactionFields) Destroy(value AssetTransferTransactionFields) {
	value.Destroy()
}


type PayTransactionFields struct {
	Receiver Address
	Amount uint64
	CloseRemainderTo *Address
}

func (r *PayTransactionFields) Destroy() {
		FfiDestroyerAddress{}.Destroy(r.Receiver);
		FfiDestroyerUint64{}.Destroy(r.Amount);
		FfiDestroyerOptionalAddress{}.Destroy(r.CloseRemainderTo);
}

type FfiConverterPayTransactionFields struct {}

var FfiConverterPayTransactionFieldsINSTANCE = FfiConverterPayTransactionFields{}

func (c FfiConverterPayTransactionFields) Lift(rb RustBufferI) PayTransactionFields {
	return LiftFromRustBuffer[PayTransactionFields](c, rb)
}

func (c FfiConverterPayTransactionFields) Read(reader io.Reader) PayTransactionFields {
	return PayTransactionFields {
			FfiConverterAddressINSTANCE.Read(reader),
			FfiConverterUint64INSTANCE.Read(reader),
			FfiConverterOptionalAddressINSTANCE.Read(reader),
	}
}

func (c FfiConverterPayTransactionFields) Lower(value PayTransactionFields) C.RustBuffer {
	return LowerIntoRustBuffer[PayTransactionFields](c, value)
}

func (c FfiConverterPayTransactionFields) Write(writer io.Writer, value PayTransactionFields) {
		FfiConverterAddressINSTANCE.Write(writer, value.Receiver);
		FfiConverterUint64INSTANCE.Write(writer, value.Amount);
		FfiConverterOptionalAddressINSTANCE.Write(writer, value.CloseRemainderTo);
}

type FfiDestroyerPayTransactionFields struct {}

func (_ FfiDestroyerPayTransactionFields) Destroy(value PayTransactionFields) {
	value.Destroy()
}


type Transaction struct {
	Header TransactionHeader
	PayFields *PayTransactionFields
	AssetTransferFields *AssetTransferTransactionFields
}

func (r *Transaction) Destroy() {
		FfiDestroyerTransactionHeader{}.Destroy(r.Header);
		FfiDestroyerOptionalPayTransactionFields{}.Destroy(r.PayFields);
		FfiDestroyerOptionalAssetTransferTransactionFields{}.Destroy(r.AssetTransferFields);
}

type FfiConverterTransaction struct {}

var FfiConverterTransactionINSTANCE = FfiConverterTransaction{}

func (c FfiConverterTransaction) Lift(rb RustBufferI) Transaction {
	return LiftFromRustBuffer[Transaction](c, rb)
}

func (c FfiConverterTransaction) Read(reader io.Reader) Transaction {
	return Transaction {
			FfiConverterTransactionHeaderINSTANCE.Read(reader),
			FfiConverterOptionalPayTransactionFieldsINSTANCE.Read(reader),
			FfiConverterOptionalAssetTransferTransactionFieldsINSTANCE.Read(reader),
	}
}

func (c FfiConverterTransaction) Lower(value Transaction) C.RustBuffer {
	return LowerIntoRustBuffer[Transaction](c, value)
}

func (c FfiConverterTransaction) Write(writer io.Writer, value Transaction) {
		FfiConverterTransactionHeaderINSTANCE.Write(writer, value.Header);
		FfiConverterOptionalPayTransactionFieldsINSTANCE.Write(writer, value.PayFields);
		FfiConverterOptionalAssetTransferTransactionFieldsINSTANCE.Write(writer, value.AssetTransferFields);
}

type FfiDestroyerTransaction struct {}

func (_ FfiDestroyerTransaction) Destroy(value Transaction) {
	value.Destroy()
}


// The transaction header contains the fields that can be present in any transaction.
// "Header" only indicates that these are common fields, NOT that they are the first fields in the transaction.
type TransactionHeader struct {
	// The type of transaction
	TransactionType TransactionType
	// The sender of the transaction
	Sender Address
	Fee uint64
	FirstValid uint64
	LastValid uint64
	GenesisHash *ByteBuf
	GenesisId *string
	Note *ByteBuf
	RekeyTo *Address
	Lease *ByteBuf
	Group *ByteBuf
}

func (r *TransactionHeader) Destroy() {
		FfiDestroyerTransactionType{}.Destroy(r.TransactionType);
		FfiDestroyerAddress{}.Destroy(r.Sender);
		FfiDestroyerUint64{}.Destroy(r.Fee);
		FfiDestroyerUint64{}.Destroy(r.FirstValid);
		FfiDestroyerUint64{}.Destroy(r.LastValid);
		FfiDestroyerOptionalTypeByteBuf{}.Destroy(r.GenesisHash);
		FfiDestroyerOptionalString{}.Destroy(r.GenesisId);
		FfiDestroyerOptionalTypeByteBuf{}.Destroy(r.Note);
		FfiDestroyerOptionalAddress{}.Destroy(r.RekeyTo);
		FfiDestroyerOptionalTypeByteBuf{}.Destroy(r.Lease);
		FfiDestroyerOptionalTypeByteBuf{}.Destroy(r.Group);
}

type FfiConverterTransactionHeader struct {}

var FfiConverterTransactionHeaderINSTANCE = FfiConverterTransactionHeader{}

func (c FfiConverterTransactionHeader) Lift(rb RustBufferI) TransactionHeader {
	return LiftFromRustBuffer[TransactionHeader](c, rb)
}

func (c FfiConverterTransactionHeader) Read(reader io.Reader) TransactionHeader {
	return TransactionHeader {
			FfiConverterTransactionTypeINSTANCE.Read(reader),
			FfiConverterAddressINSTANCE.Read(reader),
			FfiConverterUint64INSTANCE.Read(reader),
			FfiConverterUint64INSTANCE.Read(reader),
			FfiConverterUint64INSTANCE.Read(reader),
			FfiConverterOptionalTypeByteBufINSTANCE.Read(reader),
			FfiConverterOptionalStringINSTANCE.Read(reader),
			FfiConverterOptionalTypeByteBufINSTANCE.Read(reader),
			FfiConverterOptionalAddressINSTANCE.Read(reader),
			FfiConverterOptionalTypeByteBufINSTANCE.Read(reader),
			FfiConverterOptionalTypeByteBufINSTANCE.Read(reader),
	}
}

func (c FfiConverterTransactionHeader) Lower(value TransactionHeader) C.RustBuffer {
	return LowerIntoRustBuffer[TransactionHeader](c, value)
}

func (c FfiConverterTransactionHeader) Write(writer io.Writer, value TransactionHeader) {
		FfiConverterTransactionTypeINSTANCE.Write(writer, value.TransactionType);
		FfiConverterAddressINSTANCE.Write(writer, value.Sender);
		FfiConverterUint64INSTANCE.Write(writer, value.Fee);
		FfiConverterUint64INSTANCE.Write(writer, value.FirstValid);
		FfiConverterUint64INSTANCE.Write(writer, value.LastValid);
		FfiConverterOptionalTypeByteBufINSTANCE.Write(writer, value.GenesisHash);
		FfiConverterOptionalStringINSTANCE.Write(writer, value.GenesisId);
		FfiConverterOptionalTypeByteBufINSTANCE.Write(writer, value.Note);
		FfiConverterOptionalAddressINSTANCE.Write(writer, value.RekeyTo);
		FfiConverterOptionalTypeByteBufINSTANCE.Write(writer, value.Lease);
		FfiConverterOptionalTypeByteBufINSTANCE.Write(writer, value.Group);
}

type FfiDestroyerTransactionHeader struct {}

func (_ FfiDestroyerTransactionHeader) Destroy(value TransactionHeader) {
	value.Destroy()
}

type AlgoModelsError struct {
	err error
}

// Convience method to turn *AlgoModelsError into error
// Avoiding treating nil pointer as non nil error interface
func (err *AlgoModelsError) AsError() error {
	if err == nil {
		return nil
	} else {
		return err
	}
}

func (err AlgoModelsError) Error() string {
	return fmt.Sprintf("AlgoModelsError: %s", err.err.Error())
}

func (err AlgoModelsError) Unwrap() error {
	return err.err
}

// Err* are used for checking error type with `errors.Is`
var ErrAlgoModelsErrorEncodingError = fmt.Errorf("AlgoModelsErrorEncodingError")
var ErrAlgoModelsErrorDecodingError = fmt.Errorf("AlgoModelsErrorDecodingError")

// Variant structs
type AlgoModelsErrorEncodingError struct {
	Field0 string
}
func NewAlgoModelsErrorEncodingError(
	var0 string,
) *AlgoModelsError {
	return &AlgoModelsError { err: &AlgoModelsErrorEncodingError {
			Field0: var0,} }
}

func (e AlgoModelsErrorEncodingError) destroy() {
		FfiDestroyerString{}.Destroy(e.Field0)
}


func (err AlgoModelsErrorEncodingError) Error() string {
	return fmt.Sprint("EncodingError",
		": ",
		
		"Field0=",
		err.Field0,
	)
}

func (self AlgoModelsErrorEncodingError) Is(target error) bool {
	return target == ErrAlgoModelsErrorEncodingError
}
type AlgoModelsErrorDecodingError struct {
	Field0 string
}
func NewAlgoModelsErrorDecodingError(
	var0 string,
) *AlgoModelsError {
	return &AlgoModelsError { err: &AlgoModelsErrorDecodingError {
			Field0: var0,} }
}

func (e AlgoModelsErrorDecodingError) destroy() {
		FfiDestroyerString{}.Destroy(e.Field0)
}


func (err AlgoModelsErrorDecodingError) Error() string {
	return fmt.Sprint("DecodingError",
		": ",
		
		"Field0=",
		err.Field0,
	)
}

func (self AlgoModelsErrorDecodingError) Is(target error) bool {
	return target == ErrAlgoModelsErrorDecodingError
}

type FfiConverterAlgoModelsError struct{}

var FfiConverterAlgoModelsErrorINSTANCE = FfiConverterAlgoModelsError{}

func (c FfiConverterAlgoModelsError) Lift(eb RustBufferI) *AlgoModelsError {
	return LiftFromRustBuffer[*AlgoModelsError](c, eb)
}

func (c FfiConverterAlgoModelsError) Lower(value *AlgoModelsError) C.RustBuffer {
	return LowerIntoRustBuffer[*AlgoModelsError](c, value)
}

func (c FfiConverterAlgoModelsError) Read(reader io.Reader) *AlgoModelsError {
	errorID := readUint32(reader)

	switch errorID {
	case 1:
		return &AlgoModelsError{ &AlgoModelsErrorEncodingError{
			Field0: FfiConverterStringINSTANCE.Read(reader),
		}}
	case 2:
		return &AlgoModelsError{ &AlgoModelsErrorDecodingError{
			Field0: FfiConverterStringINSTANCE.Read(reader),
		}}
	default:
		panic(fmt.Sprintf("Unknown error code %d in FfiConverterAlgoModelsError.Read()", errorID))
	}
}

func (c FfiConverterAlgoModelsError) Write(writer io.Writer, value *AlgoModelsError) {
	switch variantValue := value.err.(type) {
		case *AlgoModelsErrorEncodingError:
			writeInt32(writer, 1)
			FfiConverterStringINSTANCE.Write(writer, variantValue.Field0)
		case *AlgoModelsErrorDecodingError:
			writeInt32(writer, 2)
			FfiConverterStringINSTANCE.Write(writer, variantValue.Field0)
		default:
			_ = variantValue
			panic(fmt.Sprintf("invalid error value `%v` in FfiConverterAlgoModelsError.Write", value))
	}
}

type FfiDestroyerAlgoModelsError struct {}

func (_ FfiDestroyerAlgoModelsError) Destroy(value *AlgoModelsError) {
	switch variantValue := value.err.(type) {
		case AlgoModelsErrorEncodingError:
			variantValue.destroy()
		case AlgoModelsErrorDecodingError:
			variantValue.destroy()
		default:
			_ = variantValue
			panic(fmt.Sprintf("invalid error value `%v` in FfiDestroyerAlgoModelsError.Destroy", value))
	}
}




type TransactionType uint

const (
	TransactionTypePayment TransactionType = 1
	TransactionTypeAssetTransfer TransactionType = 2
	TransactionTypeAssetFreeze TransactionType = 3
	TransactionTypeAssetConfig TransactionType = 4
	TransactionTypeKeyRegistration TransactionType = 5
	TransactionTypeApplicationCall TransactionType = 6
)

type FfiConverterTransactionType struct {}

var FfiConverterTransactionTypeINSTANCE = FfiConverterTransactionType{}

func (c FfiConverterTransactionType) Lift(rb RustBufferI) TransactionType {
	return LiftFromRustBuffer[TransactionType](c, rb)
}

func (c FfiConverterTransactionType) Lower(value TransactionType) C.RustBuffer {
	return LowerIntoRustBuffer[TransactionType](c, value)
}
func (FfiConverterTransactionType) Read(reader io.Reader) TransactionType {
	id := readInt32(reader)
	return TransactionType(id)
}

func (FfiConverterTransactionType) Write(writer io.Writer, value TransactionType) {
	writeInt32(writer, int32(value))
}

type FfiDestroyerTransactionType struct {}

func (_ FfiDestroyerTransactionType) Destroy(value TransactionType) {
}




type FfiConverterOptionalString struct{}

var FfiConverterOptionalStringINSTANCE = FfiConverterOptionalString{}

func (c FfiConverterOptionalString) Lift(rb RustBufferI) *string {
	return LiftFromRustBuffer[*string](c, rb)
}

func (_ FfiConverterOptionalString) Read(reader io.Reader) *string {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterStringINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalString) Lower(value *string) C.RustBuffer {
	return LowerIntoRustBuffer[*string](c, value)
}

func (_ FfiConverterOptionalString) Write(writer io.Writer, value *string) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterStringINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalString struct {}

func (_ FfiDestroyerOptionalString) Destroy(value *string) {
	if value != nil {
		FfiDestroyerString{}.Destroy(*value)
	}
}



type FfiConverterOptionalAddress struct{}

var FfiConverterOptionalAddressINSTANCE = FfiConverterOptionalAddress{}

func (c FfiConverterOptionalAddress) Lift(rb RustBufferI) *Address {
	return LiftFromRustBuffer[*Address](c, rb)
}

func (_ FfiConverterOptionalAddress) Read(reader io.Reader) *Address {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterAddressINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalAddress) Lower(value *Address) C.RustBuffer {
	return LowerIntoRustBuffer[*Address](c, value)
}

func (_ FfiConverterOptionalAddress) Write(writer io.Writer, value *Address) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterAddressINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalAddress struct {}

func (_ FfiDestroyerOptionalAddress) Destroy(value *Address) {
	if value != nil {
		FfiDestroyerAddress{}.Destroy(*value)
	}
}



type FfiConverterOptionalAssetTransferTransactionFields struct{}

var FfiConverterOptionalAssetTransferTransactionFieldsINSTANCE = FfiConverterOptionalAssetTransferTransactionFields{}

func (c FfiConverterOptionalAssetTransferTransactionFields) Lift(rb RustBufferI) *AssetTransferTransactionFields {
	return LiftFromRustBuffer[*AssetTransferTransactionFields](c, rb)
}

func (_ FfiConverterOptionalAssetTransferTransactionFields) Read(reader io.Reader) *AssetTransferTransactionFields {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterAssetTransferTransactionFieldsINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalAssetTransferTransactionFields) Lower(value *AssetTransferTransactionFields) C.RustBuffer {
	return LowerIntoRustBuffer[*AssetTransferTransactionFields](c, value)
}

func (_ FfiConverterOptionalAssetTransferTransactionFields) Write(writer io.Writer, value *AssetTransferTransactionFields) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterAssetTransferTransactionFieldsINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalAssetTransferTransactionFields struct {}

func (_ FfiDestroyerOptionalAssetTransferTransactionFields) Destroy(value *AssetTransferTransactionFields) {
	if value != nil {
		FfiDestroyerAssetTransferTransactionFields{}.Destroy(*value)
	}
}



type FfiConverterOptionalPayTransactionFields struct{}

var FfiConverterOptionalPayTransactionFieldsINSTANCE = FfiConverterOptionalPayTransactionFields{}

func (c FfiConverterOptionalPayTransactionFields) Lift(rb RustBufferI) *PayTransactionFields {
	return LiftFromRustBuffer[*PayTransactionFields](c, rb)
}

func (_ FfiConverterOptionalPayTransactionFields) Read(reader io.Reader) *PayTransactionFields {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterPayTransactionFieldsINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalPayTransactionFields) Lower(value *PayTransactionFields) C.RustBuffer {
	return LowerIntoRustBuffer[*PayTransactionFields](c, value)
}

func (_ FfiConverterOptionalPayTransactionFields) Write(writer io.Writer, value *PayTransactionFields) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterPayTransactionFieldsINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalPayTransactionFields struct {}

func (_ FfiDestroyerOptionalPayTransactionFields) Destroy(value *PayTransactionFields) {
	if value != nil {
		FfiDestroyerPayTransactionFields{}.Destroy(*value)
	}
}



type FfiConverterOptionalTypeByteBuf struct{}

var FfiConverterOptionalTypeByteBufINSTANCE = FfiConverterOptionalTypeByteBuf{}

func (c FfiConverterOptionalTypeByteBuf) Lift(rb RustBufferI) *ByteBuf {
	return LiftFromRustBuffer[*ByteBuf](c, rb)
}

func (_ FfiConverterOptionalTypeByteBuf) Read(reader io.Reader) *ByteBuf {
	if readInt8(reader) == 0 {
		return nil
	}
	temp := FfiConverterTypeByteBufINSTANCE.Read(reader)
	return &temp
}

func (c FfiConverterOptionalTypeByteBuf) Lower(value *ByteBuf) C.RustBuffer {
	return LowerIntoRustBuffer[*ByteBuf](c, value)
}

func (_ FfiConverterOptionalTypeByteBuf) Write(writer io.Writer, value *ByteBuf) {
	if value == nil {
		writeInt8(writer, 0)
	} else {
		writeInt8(writer, 1)
		FfiConverterTypeByteBufINSTANCE.Write(writer, *value)
	}
}

type FfiDestroyerOptionalTypeByteBuf struct {}

func (_ FfiDestroyerOptionalTypeByteBuf) Destroy(value *ByteBuf) {
	if value != nil {
		FfiDestroyerTypeByteBuf{}.Destroy(*value)
	}
}


/**
 * Typealias from the type name used in the UDL file to the builtin type.  This
 * is needed because the UDL type name is used in function/method signatures.
 * It's also what we have an external type that references a custom type.
 */
type ByteBuf = []byte
type FfiConverterTypeByteBuf = FfiConverterBytes
type FfiDestroyerTypeByteBuf = FfiDestroyerBytes
var FfiConverterTypeByteBufINSTANCE = FfiConverterBytes{}

func AddressFromPubKey(pubKey []byte) (Address, *AlgoModelsError) {
	_uniffiRV, _uniffiErr := rustCallWithError[AlgoModelsError](FfiConverterAlgoModelsError{},func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer {
		inner: C.uniffi_algo_models_ffi_fn_func_address_from_pub_key(FfiConverterBytesINSTANCE.Lower(pubKey),_uniffiStatus),
	}
	})
		if _uniffiErr != nil {
			var _uniffiDefaultValue Address
			return _uniffiDefaultValue, _uniffiErr
		} else {
			return FfiConverterAddressINSTANCE.Lift(_uniffiRV), _uniffiErr
		}
}

func AddressFromString(address string) (Address, *AlgoModelsError) {
	_uniffiRV, _uniffiErr := rustCallWithError[AlgoModelsError](FfiConverterAlgoModelsError{},func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer {
		inner: C.uniffi_algo_models_ffi_fn_func_address_from_string(FfiConverterStringINSTANCE.Lower(address),_uniffiStatus),
	}
	})
		if _uniffiErr != nil {
			var _uniffiDefaultValue Address
			return _uniffiDefaultValue, _uniffiErr
		} else {
			return FfiConverterAddressINSTANCE.Lift(_uniffiRV), _uniffiErr
		}
}

func AttachSignature(encodedTx []byte, signature []byte) ([]byte, *AlgoModelsError) {
	_uniffiRV, _uniffiErr := rustCallWithError[AlgoModelsError](FfiConverterAlgoModelsError{},func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer {
		inner: C.uniffi_algo_models_ffi_fn_func_attach_signature(FfiConverterBytesINSTANCE.Lower(encodedTx), FfiConverterBytesINSTANCE.Lower(signature),_uniffiStatus),
	}
	})
		if _uniffiErr != nil {
			var _uniffiDefaultValue []byte
			return _uniffiDefaultValue, _uniffiErr
		} else {
			return FfiConverterBytesINSTANCE.Lift(_uniffiRV), _uniffiErr
		}
}

func DecodeTransaction(bytes []byte) (Transaction, *AlgoModelsError) {
	_uniffiRV, _uniffiErr := rustCallWithError[AlgoModelsError](FfiConverterAlgoModelsError{},func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer {
		inner: C.uniffi_algo_models_ffi_fn_func_decode_transaction(FfiConverterBytesINSTANCE.Lower(bytes),_uniffiStatus),
	}
	})
		if _uniffiErr != nil {
			var _uniffiDefaultValue Transaction
			return _uniffiDefaultValue, _uniffiErr
		} else {
			return FfiConverterTransactionINSTANCE.Lift(_uniffiRV), _uniffiErr
		}
}

func EncodeTransaction(tx Transaction) ([]byte, *AlgoModelsError) {
	_uniffiRV, _uniffiErr := rustCallWithError[AlgoModelsError](FfiConverterAlgoModelsError{},func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer {
		inner: C.uniffi_algo_models_ffi_fn_func_encode_transaction(FfiConverterTransactionINSTANCE.Lower(tx),_uniffiStatus),
	}
	})
		if _uniffiErr != nil {
			var _uniffiDefaultValue []byte
			return _uniffiDefaultValue, _uniffiErr
		} else {
			return FfiConverterBytesINSTANCE.Lift(_uniffiRV), _uniffiErr
		}
}

// Get the transaction type from the encoded transaction.
// This is particularly useful when decoding a transaction that has an unknown type
func GetEncodedTransactionType(bytes []byte) (TransactionType, *AlgoModelsError) {
	_uniffiRV, _uniffiErr := rustCallWithError[AlgoModelsError](FfiConverterAlgoModelsError{},func(_uniffiStatus *C.RustCallStatus) RustBufferI {
		return GoRustBuffer {
		inner: C.uniffi_algo_models_ffi_fn_func_get_encoded_transaction_type(FfiConverterBytesINSTANCE.Lower(bytes),_uniffiStatus),
	}
	})
		if _uniffiErr != nil {
			var _uniffiDefaultValue TransactionType
			return _uniffiDefaultValue, _uniffiErr
		} else {
			return FfiConverterTransactionTypeINSTANCE.Lift(_uniffiRV), _uniffiErr
		}
}

