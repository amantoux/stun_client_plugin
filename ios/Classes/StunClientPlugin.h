#import <Flutter/Flutter.h>

@interface StunClientPlugin : NSObject<FlutterPlugin>
@end
// NOTE: Append the lines below to ios/Classes/GreeterPlugin.h

/**
 * Magic cookie
 */
#define MAGIC_COOKIE 554869826

/**
 * Binding method
 */
#define METHOD_BINDING 1

/**
 * A constant that represents a class request
 */
#define CLASS_REQUEST 0

/**
 * A constant that represents a class indication
 */
#define CLASS_INDICATION 16

/**
 * A constant that represents a class success response
 */
#define CLASS_SUCCESS_RESPONSE 256

/**
 * A constant that represents a class error response
 */
#define CLASS_ERROR_RESPONSE 272

/**
 * STUN header size
 */
#define HEADER_BYTE_SIZE 20

/**
 * MAPPED-ADDRESS attribute
 */
#define ATTR_MAPPED_ADDRESS 1

/**
 * XOR-MAPPED-ADDRESS attribute
 */
#define ATTR_XOR_MAPPED_ADDRESS 32

/**
 * ERROR-CODE attribute
 */
#define ATTR_ERROR_CODE 9

/**
 * SOFTWARE attribute
 */
#define ATTR_SOFTWARE 32802

/**
 * OTHER-ADDRESS attribute
 */
#define ATTR_OTHER_ADDRESS 32812

/**
 * CHANGE-REQUEST attribute
 */
#define ATTR_CHANGE_REQUEST 3

/**
 * RESPONSE-ORIGIN attribute
 */
#define ATTR_RESPONSE_ORIGIN 32811

/**
 * The "change IP" flag for the CHANGE-REQUEST attribute.
 */
#define CHANGE_REQUEST_IP_FLAG 4

/**
 * The "change port" flag for the CHANGE-REQUEST attribute.
 */
#define CHANGE_REQUEST_PORT_FLAG 2

#define FAMILY_IPV4 1

#define FAMILY_IPV6 2

typedef struct CDuration {
  long secs;
  int nanos;
} CDuration;

typedef struct COptions {
  const struct CDuration *timeout;
  const char *software;
} COptions;

typedef struct Response
{
  int status;
  const char *value;
  const char *error;
} Response;

/**
 * # Safety
 *
 * Watch out.
 */
Response get_xor_mapped_address(const char *stun_address,
                           const char *local_port,
                           struct COptions options);
