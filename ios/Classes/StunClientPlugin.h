#import <Flutter/Flutter.h>

@interface StunClientPlugin : NSObject<FlutterPlugin>
@end
// NOTE: Append the lines below to ios/Classes/GreeterPlugin.h

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

Response get_xor_mapped_address(const char *stun_address, struct COptions options);
