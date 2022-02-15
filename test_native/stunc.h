typedef struct CDuration
{
  long secs;
  int nanos;
} CDuration;

typedef struct COptions
{
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