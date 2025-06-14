export enum HttpMethod {
    GET = "GET",
    POST = "POST",
    PUT = "PUT",
    DELETE = "DELETE",
    PATCH = "PATCH",
    OPTIONS = "OPTIONS",
    HEAD = "HEAD"
}
const HTTP_HEADERS = {
    CONTENT_TYPE: "Content-Type",
    AUTHORIZATION: "Authorization",
    ACCEPT: "Accept",
    CACHE_CONTROL: "Cache-Control",
    X_REQUESTED_WITH: "X-Requested-With"
} as const;

export type HttpHeaderName = typeof HTTP_HEADERS[keyof typeof HTTP_HEADERS];
export type HttpHeaderValue = string;


// Builder class for crafting http requests
export class HttpRequestBuilder {

    private requestInit: RequestInit = {
        headers: {}
    };

    public method(method: HttpMethod): HttpRequestBuilder {
        this.requestInit.method = method;
        return this;
    }

    public header(key: string | HttpHeaderName, value: HttpHeaderValue): HttpRequestBuilder {
        if (!this.requestInit.headers) {
            this.requestInit.headers = {};
        }
        
        // If it's already a valid HttpHeaderName, use it directly
        if (Object.values(HTTP_HEADERS).includes(key as any)) {
            (this.requestInit.headers as Record<HttpHeaderName, HttpHeaderValue>)[key] = value;
            return this;
        }
        
        // Otherwise, try to find a case-insensitive match
        const lowerKey = key.toLowerCase();
        const canonicalKey = Object.values(HTTP_HEADERS).find(
            headerKey => headerKey.toLowerCase() === lowerKey
        ) as HttpHeaderName | undefined;
        
        if (canonicalKey) {
            (this.requestInit.headers as Record<HttpHeaderName, HttpHeaderValue>)[canonicalKey] = value;
        } else {
            // Handle unrecognized header (optional)
            console.warn(`Using non-standard header: ${key}`);
            (this.requestInit.headers as Record<HttpHeaderName, HttpHeaderValue>)[key] = value;
        }
        
        return this;
    }

    public contentType(value: HttpHeaderValue): HttpRequestBuilder {
        return this.header(HTTP_HEADERS.CONTENT_TYPE, value);
    }

    public jsonContentType(): HttpRequestBuilder {
        return this.contentType("application/json");
    }

    public authorization(value: HttpHeaderValue): HttpRequestBuilder {
        return this.header(HTTP_HEADERS.AUTHORIZATION, value);
    }

    public body(data: any, shouldStringtify: boolean = true): HttpRequestBuilder {
        this.requestInit.body = shouldStringtify ? JSON.stringify(data) : data;
        return this;
    }

    public build(): RequestInit {
        return { ...this.requestInit };
    }

    public static get(): HttpRequestBuilder {
        return new HttpRequestBuilder().method(HttpMethod.GET);
    }

    public static post(): HttpRequestBuilder {
        return new HttpRequestBuilder().method(HttpMethod.POST);
    }
}

// Class for simplifying and standardizing making HTTP requests.
// It is built ontop of the HttpRequestBuilder and native fetch api
export default class HttpClient {
    private url: string = "";
    private builder: HttpRequestBuilder

    constructor(url: string, builder?: HttpRequestBuilder) {
        this.url = url
        this.builder = builder || new HttpRequestBuilder();
    }

    public createCustomRequest(): HttpRequestBuilder {
        return new HttpRequestBuilder()
    }


    public async request<T>(config: RequestInit): Promise<T> {

        console.log(this.url)
        const response = await fetch(this.url, config);

        // If an error occurs, we will handle it here
        if (!response.ok) {
            throw new Error(`Http Status: ${await response.text()}`);
        }

        return response.json();
    }

    public async get<T>(): Promise<T> {
        const requestBuilder = this.builder.method(HttpMethod.GET);
        return this.request<T>(requestBuilder.build());
    }

    public async post<T>(data?: any, builder?: HttpRequestBuilder): Promise<T> {
        const requestBuilder = builder || this.builder.method(HttpMethod.POST).jsonContentType();

        if (data) {
            requestBuilder.body(data);
        }

        console.log(requestBuilder.build())
        return this.request<T>(requestBuilder.build());
    }

}